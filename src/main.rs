use config::{load_radio, Radio};
use libc::{
    getsockopt, setns, setsockopt, socklen_t, CLONE_NEWNET, CLONE_NEWUSER, CLONE_NEWUTS,
    SOL_SOCKET, SO_BINDTODEVICE,
};
use mac80211_hwsim::MACAddress;
use memmap2::MmapMut;
use memmap2::MmapOptions;
use netlink_packet_utils::byteorder::ByteOrder;
use netlink_packet_utils::byteorder::NativeEndian;
use netlink_packet_utils::parsers::parse_u32;
use netlink_packet_utils::Emitable;
use netns_rs::get_from_current_thread;
use netns_rs::NetNs;
use std::collections::HashMap;
use std::env;
use std::io;
use std::os::fd::AsRawFd;
use std::time::Duration;
use std::{ffi::CString, os::fd::RawFd};
use structs::GenlFrameRX;
use tokio::fs::File;
use tokio::fs::OpenOptions;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::time::sleep;

use anyhow::Error;
use futures::StreamExt;
use genetlink::{self, new_connection};
use mac80211_hwsim::constants::ETH_ALEN;
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;
use netlink_packet_utils::ParseableParametrized;
use netlink_sys::{
    protocols::{NETLINK_GENERIC, NETLINK_ROUTE},
    Socket, SocketAddr,
};

use crate::structs::GenlFrameTX;
use crate::structs::GenlTXInfoFrame;
use crate::{
    config::{load_config, Config},
    mac80211_hwsim::{
        constants::MICROSECONDS_TO_NANOSECONDS, ctrl::nlas::HwsimAttrs, new_radio_nl,
        structs::ReceiverInfo,
    },
    structs::{GenlNewRadio, GenlRadioOps, GenlRegister},
};

use self::mac80211_hwsim::ctrl::*;

mod config;
mod mac80211_hwsim;
mod structs;

pub const MAX_PAGE_NUM_PER_RADIO: usize = 64;
pub const MAX_PAGE_ORDER_PER_RADIO: usize = 15;
pub const DEFAULT_PAGE_ORDER_PER_RADIO: usize = 4;
pub const PAGE_SHIFT: usize = 12;
pub const PAGE_SIZE: usize = 1 << PAGE_SHIFT;

#[derive(Clone, Debug, Default)]
struct SharedMemoryBuffer {
    page_order: usize,
    page_size: usize,

    tx_offset: usize,
    tx_size: usize,
    tx_page_offset: usize,
    tx_page_size: usize,

    rx_offset: usize,
    rx_size: usize,
    rx_page_offset: usize,
    rx_page_size: usize,

    tx_end: usize,
}

impl SharedMemoryBuffer {
    fn read_data_from_pointer_by_len(
        &self,
        buffer: &[u8],
        pointer: usize,
        len: usize,
    ) -> (Box<[u8]>, usize) {
        let mut boxed_slice: Box<[u8]> = vec![0; len].into_boxed_slice();

        let l = self.rx_offset + pointer;
        let r = self.rx_offset + (pointer + len) % self.rx_size;

        // println!("{}", len);

        if l > r {
            boxed_slice[0..(self.rx_size - pointer)]
                .copy_from_slice(&buffer[l..(self.rx_offset + self.rx_size)]);
            boxed_slice[(self.rx_size - pointer)..].copy_from_slice(&buffer[self.rx_offset..r]);
        } else {
            boxed_slice.copy_from_slice(&buffer[l..r]);
        }

        (boxed_slice, r - self.rx_offset)
    }

    fn parse_data(&self, buffer: &[u8], pointer: usize) -> Result<Box<GenlFrameTX>, Error> {
        let (length_slice, new_pointer) = self.read_data_from_pointer_by_len(buffer, pointer, 4);
        let length = parse_u32(&length_slice).expect("cannot parse length");

        if length > PAGE_SIZE as u32 {
            return Err(anyhow::anyhow!("Len invalid"));
        }

        let (raw_data, _) =
            self.read_data_from_pointer_by_len(buffer, new_pointer, length as usize);

        let raw_nlas = parse_nlas(&raw_data)?;

        Ok(Box::<GenlFrameTX>::new(parse_genl_message::<GenlFrameTX>(
            GenlMAC {
                cmd: HwsimCmd::Frame,
                nlas: raw_nlas,
            },
        )))
    }

    fn write_data_to_pointer_by_len(
        &mut self,
        shared_memory: &mut [u8],
        buffer: &[u8],
        len: usize,
    ) -> usize {
        let l = self.tx_offset + self.tx_end;
        let r = self.tx_offset + (self.tx_end + len) % self.tx_size;

        if l > r {
            shared_memory[l..(self.tx_offset + self.tx_size)]
                .copy_from_slice(&buffer[0..(self.tx_size - self.tx_end)]);
            shared_memory[self.tx_offset..r]
                .copy_from_slice(&buffer[(self.tx_size - self.tx_end)..]);
        } else {
            shared_memory[l..r].copy_from_slice(buffer);
        }

        self.tx_end = (self.tx_end + len) % self.tx_size;

        r - self.tx_offset
    }

    fn emit_data(&mut self, shared_memory: &mut [u8], data: GenlFrameRX) -> usize {
        let genl_data: GenlMAC = data.try_into().expect("fail to into GenlMAC");
        let length = genl_data.buffer_len();

        let tx_end = self.tx_end;

        let mut raw_data: Box<[u8]> = vec![0; length].into_boxed_slice();
        genl_data.emit(&mut raw_data);

        let mut length_buffer: Box<[u8]> = vec![0; 4].into_boxed_slice();
        NativeEndian::write_u32(&mut length_buffer, length as u32);

        self.write_data_to_pointer_by_len(shared_memory, &length_buffer, 4);
        self.write_data_to_pointer_by_len(shared_memory, &raw_data, length);

        tx_end
    }
}

#[derive(Clone, Debug)]
struct RadioInfo {
    radio: Radio,
    sm_buffer: SharedMemoryBuffer,
    tx: UnboundedSender<GenlFrameTX>,
}

#[derive(Clone, Debug)]
struct TXInfo {
    tx: UnboundedSender<GenlFrameTX>,
    mac: HwsimRadio,
}

#[tokio::main]
async fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();

    // 检查参数数量
    if args.len() < 2 {
        println!("请提供一个整数作为参数");
        return;
    }

    // 解析参数为整数
    let node_id: i32 = match args[1].parse() {
        Ok(n) => n,
        Err(_) => {
            println!("参数不是有效的整数");
            return;
        }
    };

    let config_path = "./config/topology.yaml";
    let config = load_config(config_path);

    let num = config.radios.len();

    println!("Ready to spawn {} radios.", num);

    let mut radio_infos: HashMap<usize, RadioInfo> = HashMap::new();

    let mut rxs: HashMap<usize, UnboundedReceiver<GenlFrameTX>> = HashMap::new();

    for i in 0..num {
        let (tx, rx) = mpsc::unbounded_channel::<GenlFrameTX>();
        rxs.insert(config.radios[i].id, rx);

        let mut sm_buffer = SharedMemoryBuffer::default();
        sm_buffer.page_order = DEFAULT_PAGE_ORDER_PER_RADIO;
        sm_buffer.page_size = 1 << DEFAULT_PAGE_ORDER_PER_RADIO;

        sm_buffer.rx_page_offset = 0;
        sm_buffer.rx_page_size = sm_buffer.page_size / 2;
        sm_buffer.rx_offset = sm_buffer.rx_page_offset << PAGE_SHIFT;
        sm_buffer.rx_size = sm_buffer.rx_page_size << PAGE_SHIFT;

        sm_buffer.tx_page_offset = sm_buffer.rx_page_offset + sm_buffer.rx_page_size;
        sm_buffer.tx_page_size = sm_buffer.page_size - sm_buffer.rx_page_size;
        sm_buffer.tx_offset = sm_buffer.tx_page_offset << PAGE_SHIFT;
        sm_buffer.tx_size = sm_buffer.tx_page_size << PAGE_SHIFT;

        radio_infos.insert(
            config.radios[i].id,
            RadioInfo {
                radio: config.radios[i].clone(),
                sm_buffer,
                tx,
            },
        );
    }

    let (terminate_tx, terminate_rx) = broadcast::channel::<()>(num as usize);

    let handles = radio_infos
        .iter()
        .map(|(id, radio_info)| {
            let mut txs: Vec<TXInfo> = vec![];

            config.links.iter().for_each(|link| {
                if link.src == *id {
                    let info = radio_infos.get(&link.dst);
                    txs.push(TXInfo {
                        tx: info.unwrap().tx.clone(),
                        mac: HwsimRadio {
                            addr: info.unwrap().radio.perm_addr.clone(),
                            hw_addr: info.unwrap().radio.perm_addr.clone(),
                        },
                    });
                } else if link.dst == *id && link.mutual {
                    let info = radio_infos.get(&link.src);
                    txs.push(TXInfo {
                        tx: info.unwrap().tx.clone(),
                        mac: HwsimRadio {
                            addr: info.unwrap().radio.perm_addr.clone(),
                            hw_addr: info.unwrap().radio.perm_addr.clone(),
                        },
                    });
                }
            });
            // println!("{}", id);
            // println!("{:?}", &txs);
            // println!("{:?}", &radio_info);

            tokio::spawn(radio_process(
                *id,
                radio_info.clone(),
                txs,
                rxs.remove(id).unwrap(),
                terminate_rx.resubscribe(),
            ))
        })
        .collect::<Vec<_>>();

    let mut sigint = signal(SignalKind::interrupt()).unwrap();

    tokio::select! {
        _ = sigint.recv() => {
            println!("Received Ctrl+C signal. Performing cleanup...");

            let _ = terminate_tx.send(());

        }
    }

    for handle in handles {
        handle.await.unwrap();
    }

    // Perform clean after all threads broke down.

    println!("Exiting...");
}

#[derive(Clone, Debug)]
pub struct HwsimRadio {
    addr: MACAddress,
    hw_addr: MACAddress,
}

#[derive(Default)]
pub struct HwsimRadios {
    radios: Vec<HwsimRadio>,
}

macro_rules! netns_name {
    ($id:expr) => {
        format!("wmediumd-net-{}", $id)
    };
}

macro_rules! radio_name {
    ($netns:expr, $id:literal) => {
        format!("radio-{}-{}", $netns, $id)
    };
}

macro_rules! mac_array {
    ($netns:expr, $id:literal) => {{
        let mut arr: [u8; 6] = [0x66, 0x0, 0x0, 0x0, 0x0, 0x0];
        arr[4] = $netns;
        arr[5] = $id;
        arr
    }};
}

fn print_hex(slice: &[u8], offset: usize, len: usize) {
    let end = offset + len;
    if end > slice.len() {
        return;
    }
    print!("HEX: ----- [");

    for i in offset..end {
        print!("{:02X} ", slice[i]);
    }
    println!("] -----");
}

async fn radio_process(
    id: usize,
    mut radio_info: RadioInfo,
    txs: Vec<TXInfo>,
    mut rx: UnboundedReceiver<GenlFrameTX>,
    mut terminate_rx: broadcast::Receiver<()>,
) {
    println!("Spawning the {} link.", &id);
    // dbg!(template);

    // let tx_lock: Mutex<usize> = Mutex::new(1);

    // Prepare netns and ..

    let net_ns: NetNs;
    let result = NetNs::get(netns_name!(&id));

    match result {
        Ok(v) => {
            net_ns = v;
        }
        Err(_) => {
            let result = NetNs::new(netns_name!(&id));
            match result {
                Ok(v) => {
                    net_ns = v;
                }
                Err(_) => {
                    println!("Link {} cannot create netns.", &id + 1);
                    return;
                }
            };
        }
    };

    // Enter the new netns
    match net_ns.enter() {
        Ok(_) => {}
        Err(_) => {
            println!("Link {} cannot enter netns.", &id + 1);
            return;
        }
    };

    let (conn, mut handle, mut receiver) = new_connection().expect("Failed to create connection.");
    tokio::spawn(conn);

    // Register wmediumd using genetlink
    let genl_register = GenlRegister {};
    handle
        .notify(genl_register.generate_genl_message())
        .await
        .expect("Failed to register wmediumd");

    // Create the simple link which includes 2 radios lied in the same netns.

    let new_radio: GenlNewRadio = radio_info
        .radio
        .clone()
        .try_into()
        .expect("handle radio conversion error");

    new_radio_nl(&mut handle, new_radio).await;

    let dev_file_path = "/dev/".to_string() + &radio_info.radio.radio_name.clone();
    sleep(Duration::from_secs(1)).await;
    // println!("{}", &dev_file_path);

    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&dev_file_path)
        .await
        .expect("Failed to open dev");

    // println!("{}", (2 << DEFAULT_PAGE_ORDER_PER_RADIO) * 4096);
    // file.set_len((2 << DEFAULT_PAGE_ORDER_PER_RADIO) * 4096)
    //     .await
    //     .expect("Failed to set len");
    // println!("{}", radio_info.sm_buffer.page_size as usize);
    let mut mmap = unsafe {
        // MmapMut::map_mut(&file).expect("Failed to mmap")
        MmapOptions::new()
            .len(radio_info.sm_buffer.page_size as usize * PAGE_SIZE)
            .map_mut(&file)
            .expect("Failed to mmap")
    };

    loop {
        // let ns = get_from_current_thread().unwrap();
        // println!("{:?}", ns.file().metadata());
        // println!("{}, {:?}", id, get_from_current_thread().unwrap());
        tokio::select! {
            // _ = tokio::time::sleep(Duration::from_secs(1)) => {
            //     // 执行任务逻辑
            //     println!("Long running task {} is running...", id);
            // }
            Some(msg) = receiver.next() => {
                let msg = msg.0;
                match msg.payload {
                    netlink_packet_core::NetlinkPayload::InnerMessage(msg) => {
                        let v = GenlMAC::parse_with_param(&msg.payload, msg.header);
                        // dbg!(&v);
                        // println!("{:?}", &v);
                        // println!("{}", msg.payload.len());
                        match v {
                            Ok(frame) => {
                                if frame.cmd != HwsimCmd::Frame {
                                    // println!("{:?}", &frame.cmd);
                                    continue;
                                }

                                // println!("{:?}", &frame.nlas);

                                let genl_data = parse_genl_message::<GenlFrameTX>(frame);


                                let data;

                                match radio_info.sm_buffer.parse_data(&mmap, genl_data.shared_memory_pointer as usize) {
                                    Ok(v) => data = v,
                                    Err(_) => {
                                        println!("fail to parse");
                                        continue;
                                    },
                                }

                                // println!("{}: {}", &id, &genl_data.shared_memory_pointer);

                                // println!("len: {}", len);
                                // print_hex(&mmap, po + 4, len.try_into().unwrap());

                                // let data = parse_genl_message::<GenlFrameTX>(GenlMAC { cmd: HwsimCmd::Frame, nlas: raw_data });
                                // println!("{:?}", data);
                                if data.frame.header.addr1.eq(&[255, 255, 255, 255, 255, 255])
                                    || data.frame.header.addr1.eq(&[0, 0, 0, 0, 0, 0]) {
                                    txs.iter().for_each(|tx| {

                                        let result = tx.tx.send(*data.clone());
                                        match result {
                                            Ok(_) => {
                                                // println!("mpsc send success");
                                            },
                                            Err(_) => {
                                                println!("mpsc send fail");
                                            },
                                        }
                                    })
                                } else {
                                    for tx in &txs {
                                        if tx.mac.addr.eq(&data.frame.header.addr1) {
                                            assert!(&tx.mac.hw_addr.ne(&radio_info.radio.perm_addr));
                                            let result = tx.tx.send(*data.clone());
                                            match result {
                                                Ok(_) => {},
                                                Err(_) => {
                                                    println!("mpsc send fail");
                                                },
                                            }
                                            break;
                                        }
                                    }
                                }
                                // println!("1");
                                // println!("{} -- {:?}", id, data.frame_header.addr1);


                                // println!("{:?}", &rx_info);

                            }
                            Err(_) => {}
                        }
                    }
                    _ => {}
                }
            }
            Some(msg) = rx.recv() => {

                let signal = (30 - 91) as u32;

                let mut frame_rx_data = GenlFrameRX::default();
                let mut tx_info_frame = GenlTXInfoFrame::default();
                tx_info_frame.addr_transmitter = msg.addr_transmitter;
                tx_info_frame.flags = msg.flags;
                frame_rx_data.rx_rate = msg.tx_info[0].idx as u32;
                frame_rx_data.signal = signal;
                tx_info_frame.tx_info = msg.tx_info;
                tx_info_frame.cookie = msg.cookie;
                frame_rx_data.freq = msg.freq;
                frame_rx_data.frame = msg.frame;
                frame_rx_data.addr_receiver = radio_info.radio.perm_addr.clone();

                let pointer = radio_info.sm_buffer.emit_data(&mut mmap, frame_rx_data);

                let mut frame_rx_nl = GenlFrameRX::default();

                frame_rx_nl.shared_memory_pointer = pointer as u64;
                frame_rx_nl.addr_receiver = radio_info.radio.perm_addr.clone();


                // println!("{:?}", &frame_rx.addr_receiver);
                // println!("{:?}", &tx_info_frame.addr_transmitter);
                // assert!(&frame_rx.addr_receiver.ne(&tx_info_frame.addr_transmitter));

                match handle.notify(frame_rx_nl.generate_genl_message()).await {
                    Ok(_) => {
                        // println!("handle 1 frame rx");

                    }
                    Err(_) => {
                        println!("fail frame rx");
                    }
                }

                match handle.notify(tx_info_frame.generate_genl_message()).await {
                    Ok(_) => {
                        // println!("handle 1 frame tx info");

                    }
                    Err(_) => {
                        println!("fail frame tx info");
                    }
                }
            }
            _ = terminate_rx.recv() => {
                // 接收到终止信号，退出循环
                println!("Terminating link thread {}...", id);

                let result = NetNs::remove(net_ns);
                match result {
                    Err(_) => {
                        println!("Netns {} cannot remove.", &id + 1);
                    },
                    _ => {}
                }
                break;
            }
        }
    }
}
