use config::{load_radio, Radio};
use libc::{
    getsockopt, setns, setsockopt, socklen_t, CLONE_NEWNET, CLONE_NEWUSER, CLONE_NEWUTS,
    SOL_SOCKET, SO_BINDTODEVICE,
};
use mac80211_hwsim::MACAddress;
use netns_rs::get_from_current_thread;
use netns_rs::NetNs;
use std::collections::HashMap;
use std::env;
use std::io;
use std::os::fd::AsRawFd;
use std::time::Duration;
use std::{ffi::CString, os::fd::RawFd};
use structs::GenlFrameRX;
use tokio::signal::unix::{signal, SignalKind};
use tokio::sync::broadcast::{self, Receiver, Sender};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};
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

#[derive(Clone, Debug)]
struct RadioInfo {
    radio: Radio,
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
        radio_infos.insert(
            config.radios[i].id,
            RadioInfo {
                radio: config.radios[i].clone(),
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
            println!("{}", id);
            println!("{:?}", &txs);
            println!("{:?}", &radio_info);

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

async fn radio_process(
    id: usize,
    radio_info: RadioInfo,
    txs: Vec<TXInfo>,
    mut rx: UnboundedReceiver<GenlFrameTX>,
    mut terminate_rx: broadcast::Receiver<()>,
) {
    println!("Spawning the {} link.", &id);
    // dbg!(template);

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
                        match v {
                            Ok(frame) => {
                                if frame.cmd != HwsimCmd::Frame {
                                    // println!("{:?}", &frame.cmd);
                                    continue;
                                }

                                let data = parse_genl_message::<GenlFrameTX>(frame);

                                if data.frame.header.addr1.eq(&[255, 255, 255, 255, 255, 255])
                                    || data.frame.header.addr1.eq(&[0, 0, 0, 0, 0, 0]) {
                                    txs.iter().for_each(|tx| {

                                        let result = tx.tx.send(data.clone());
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
                                            let result = tx.tx.send(data.clone());
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
                                let signal = (30 - 91) as u32;

                                let mut tx_info_frame = GenlTXInfoFrame::default();
                                tx_info_frame.addr_transmitter = data.addr_transmitter;
                                tx_info_frame.flags = data.flags;
                                tx_info_frame.tx_info = data.tx_info;
                                tx_info_frame.cookie = data.cookie;
                                tx_info_frame.signal = signal;
                                match handle.notify(tx_info_frame.generate_genl_message()).await {
                                    Ok(_) => {
                                        // println!("handle 1 frame tx info");

                                    }
                                    Err(_) => {
                                        println!("fail frame tx info");
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

                let mut frame_rx = GenlFrameRX::default();

                frame_rx.rx_rate = msg.tx_info[0].idx as u32;
                frame_rx.signal = signal;

                frame_rx.freq = msg.freq;
                frame_rx.frame = msg.frame;
                frame_rx.addr_receiver = radio_info.radio.perm_addr.clone();


                // println!("{:?}", &frame_rx.addr_receiver);
                // println!("{:?}", &tx_info_frame.addr_transmitter);
                // assert!(&frame_rx.addr_receiver.ne(&tx_info_frame.addr_transmitter));


                match handle.notify(frame_rx.generate_genl_message()).await {
                    Ok(_) => {
                        // println!("handle 1 frame rx");

                    }
                    Err(_) => {
                        println!("fail frame rx");
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
