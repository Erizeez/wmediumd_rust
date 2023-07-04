use config::{load_radio, Radio};
use libc::{
    getsockopt, setns, setsockopt, socklen_t, CLONE_NEWNET, CLONE_NEWUSER, CLONE_NEWUTS,
    SOL_SOCKET, SO_BINDTODEVICE,
};
use netns_rs::NetNs;
use std::collections::HashMap;
use std::env;
use std::os::fd::AsRawFd;
use std::time::Duration;
use std::{ffi::CString, os::fd::RawFd};
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

use crate::{
    config::{load_config, Config},
    mac80211_hwsim::{
        constants::MICROSECONDS_TO_NANOSECONDS, ctrl::nlas::HwsimAttrs, new_radio_nl,
        structs::ReceiverInfo,
    },
    structs::{GenlNewRadio, GenlRadioOps, GenlRegister, GenlYawmdRXInfo, GenlYawmdTXInfo},
};

use self::mac80211_hwsim::ctrl::*;

mod config;
mod mac80211_hwsim;
mod structs;

#[derive(Clone)]
struct RadioInfo {
    radio: Radio,
    tx: UnboundedSender<GenlYawmdRXInfo>,
}

#[derive(Clone, Debug)]
struct TXInfo {
    tx: UnboundedSender<GenlYawmdRXInfo>,
    mac: HwsimRadio,
}

#[tokio::main]
async fn main() {
    // tokio::spawn(init_genetlink("./config/first_link.yaml", "ns0"));

    let config_path = "./config/topology.yaml";
    let config = load_config(config_path);

    let num = config.radios.len();

    println!("Ready to spawn {} radios.", num);

    let mut radio_infos: HashMap<usize, RadioInfo> = HashMap::new();

    let mut rxs: HashMap<usize, UnboundedReceiver<GenlYawmdRXInfo>> = HashMap::new();

    for i in 0..num {
        let (tx, rx) = mpsc::unbounded_channel::<GenlYawmdRXInfo>();
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
            // println!("{}", id);
            config.links.iter().for_each(|link| {
                if link.src == *id {
                    let info = radio_infos.get(&link.dst);
                    txs.push(TXInfo {
                        tx: radio_info.tx.clone(),
                        mac: HwsimRadio {
                            addr: info.unwrap().radio.perm_addr.clone(),
                            hw_addr: info.unwrap().radio.perm_addr.clone(),
                        },
                    });
                } else if link.dst == *id && link.mutual {
                    let info = radio_infos.get(&link.src);
                    txs.push(TXInfo {
                        tx: radio_info.tx.clone(),
                        mac: HwsimRadio {
                            addr: info.unwrap().radio.perm_addr.clone(),
                            hw_addr: info.unwrap().radio.perm_addr.clone(),
                        },
                    });
                }
            });

            // println!("{:?}", txs);

            tokio::spawn(radio_process(
                *id,
                (*radio_info).clone(),
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

type MACAddress = [u8; ETH_ALEN];

#[derive(Clone, Debug)]
pub struct HwsimRadio {
    addr: MACAddress,
    hw_addr: MACAddress,
}

#[derive(Default)]
pub struct HwsimRadios {
    radios: Vec<HwsimRadio>,
}

impl HwsimRadios {
    fn get_hwaddr_by_addr(&self, dst: &MACAddress, src: &MACAddress) -> Option<MACAddress> {
        if dst.eq(&[255, 255, 255, 255, 255, 255]) {
            for radio in &self.radios {
                if radio.addr.ne(src) {
                    return Some(radio.hw_addr.clone());
                }
            }
        }
        for radio in &self.radios {
            if radio.addr.eq(dst) {
                return Some(radio.hw_addr.clone());
            }
        }
        None
    }
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
    mut rx: UnboundedReceiver<GenlYawmdRXInfo>,
    mut terminate_rx: broadcast::Receiver<()>,
) {
    println!("Spawning the {} link.", &id + 1);
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

    let mut hwsim_radios = HwsimRadios::default();
    // hwsim_radios.radios.push(HwsimRadio {
    //     addr: radio1.perm_addr.clone(),
    //     hw_addr: radio1.perm_addr.clone(),
    // });
    // hwsim_radios.radios.push(HwsimRadio {
    //     addr: radio2.perm_addr.clone(),
    //     hw_addr: radio2.perm_addr.clone(),
    // });

    new_radio_nl(&mut handle, new_radio).await;

    loop {
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
                        match v {
                            Ok(frame) => {
                                if frame.cmd != HwsimCmd::YawmdTXInfo {
                                    continue;
                                }

                                let signal = (30 - 91) as u32;

                                let data = parse_genl_message::<GenlYawmdTXInfo>(frame);

                                let mut rx_info = GenlYawmdRXInfo::default();
                                rx_info.addr_transmitter = data.addr_transmitter;
                                rx_info.flags = data.flags;
                                rx_info.rx_rate = data.tx_info[0].idx as u32;
                                rx_info.signal = signal;
                                rx_info.tx_info = data.tx_info;
                                rx_info.cookie = data.cookie;
                                rx_info.freq = data.freq;
                                rx_info.timestamp = data.timestamp;
                                rx_info.receiver_info.signal = signal;

                                // println!("1");

                                if data.frame_header.addr1.eq(&[255, 255, 255, 255, 255, 255])
                                    || data.frame_header.addr1.eq(&[0, 0, 0, 0, 0, 0]) {
                                    txs.iter().for_each(|tx| {
                                        // if tx.mac.addr.ne(&data.frame_header.addr2) {
                                            rx_info.receiver_info.addr = tx.mac.hw_addr.clone();
                                            let result = tx.tx.send(rx_info.clone());
                                            match result {
                                                Ok(_) => {
                                                    // println!("mpsc send success");
                                                },
                                                Err(_) => {
                                                    println!("mpsc send fail");
                                                },
                                            }
                                        // }
                                    })
                                } else {
                                    for tx in &txs {
                                        if tx.mac.addr.eq(&data.frame_header.addr1) {
                                            rx_info.receiver_info.addr = tx.mac.hw_addr.clone();
                                            let result = tx.tx.send(rx_info.clone());
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
                                // println!("{:?}", &rx_info);

                            }
                            Err(_) => {}
                        }
                    }
                    _ => {}
                }
            }
            Some(msg) = rx.recv() => {
                println!("{:?}", &msg);
                match handle.notify(msg.generate_genl_message()).await {
                    Ok(_) => {
                        // println!("handle 1 msg");

                    }
                    Err(_) => {
                        println!("fail-------------");
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

async fn init_genetlink(config_path: &str, net_ns: &str) -> Result<(), Error> {
    println!("Start Genetlink");

    // let config: Config = load_config(config_path);

    let genl_register = GenlRegister {};

    let (conn, mut handle, mut receiver) = new_connection()?;

    tokio::spawn(conn);

    handle.notify(genl_register.generate_genl_message()).await?;

    // let ops = GenlRadioOps { idx: 1 };
    // match handle.request(ops.generate_genl_get()).await {
    //     Ok(mut stream) => {
    //         while let Some(msg) = stream.next().await {
    //             match msg {
    //                 Ok(v) => match v.payload {
    //                     NetlinkPayload::InnerMessage(v) => {
    //                         println!("{:?}", v.payload);
    //                     }
    //                     _ => {
    //                         println!("error");
    //                     }
    //                 },
    //                 Err(v) => {
    //                     println!("{:?}", v);
    //                 }
    //             }
    //         }
    //     }
    //     Err(v) => {
    //         println!("{:?}", v);
    //     }
    // };

    // for radio in &config.radios {
    //     new_radio_nl(&mut handle, &radio).await;
    // }
    let mut hwsim_radios: HwsimRadios = HwsimRadios::default();

    // config
    //     .try_into()
    //     .expect("handle hwsim_radio conversion error");

    hwsim_radios.radios.push(HwsimRadio {
        addr: [0x2, 0x0, 0x0, 0x0, 0x0, 0x0],
        hw_addr: [0x42, 0x0, 0x0, 0x0, 0x0, 0x0],
    });

    hwsim_radios.radios.push(HwsimRadio {
        addr: [0x2, 0x0, 0x0, 0x0, 0x1, 0x0],
        hw_addr: [0x42, 0x0, 0x0, 0x0, 0x1, 0x0],
    });

    // let ops = GenlRadioOps::default();
    // match handle.request(ops.generate_genl_dump()).await {
    //     Ok(mut stream) => {
    //         while let Some(msg) = stream.next().await {
    //             match msg {
    //                 Ok(v) => match v.payload {
    //                     NetlinkPayload::InnerMessage(v) => {
    //                         println!("{:?}", v.payload);
    //                     }
    //                     _ => {
    //                         println!("error");
    //                     }
    //                 },
    //                 Err(v) => {
    //                     println!("{:?}", v);
    //                 }
    //             }
    //         }
    //     }
    //     Err(v) => {
    //         println!("{:?}", v);
    //     }
    // };
    // return Ok(());

    while let Some(msg) = receiver.next().await {
        let msg = msg.0;
        match msg.payload {
            netlink_packet_core::NetlinkPayload::InnerMessage(msg) => {
                let v = GenlMAC::parse_with_param(&msg.payload, msg.header);
                // dbg!(&v);
                match v {
                    Ok(frame) => {
                        if frame.cmd != HwsimCmd::YawmdTXInfo {
                            // println!("{:?}-- {:?} ", frame.cmd, frame.nlas);
                            continue;
                        }

                        let signal = (30 - 91) as u32;

                        let data = parse_genl_message::<GenlYawmdTXInfo>(frame);

                        let mut rx_info = GenlYawmdRXInfo::default();
                        rx_info.addr_transmitter = data.addr_transmitter;
                        rx_info.flags = data.flags;
                        rx_info.rx_rate = data.tx_info[0].idx as u32;
                        rx_info.signal = signal;
                        rx_info.tx_info = data.tx_info;
                        rx_info.cookie = data.cookie;
                        rx_info.freq = data.freq;
                        rx_info.timestamp = data.timestamp;

                        let mut receiver_info = ReceiverInfo::default();
                        // if data.addr_transmitter.eq(&[0x42, 0x0, 0x0, 0x0, 0x1, 0x0]) {
                        //     receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x0, 0x0];
                        // } else {
                        //     receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
                        // }

                        match &hwsim_radios
                            .get_hwaddr_by_addr(&data.frame_header.addr1, &data.frame_header.addr2)
                        {
                            Some(v) => {
                                receiver_info.addr = v.clone();
                            }
                            None => {}
                        }

                        receiver_info.signal = signal;
                        rx_info.receiver_info = receiver_info;

                        // println!("{:?}", &rx_info);

                        match handle.notify(rx_info.generate_genl_message()).await {
                            Ok(_) => {}
                            Err(_) => {}
                        }
                    }
                    Err(_) => {}
                }
            }
            _ => {}
        }
    }
    // #[for_await]
    // for msg in receiver {
    //     let msg = msg?;
    //     println!("{}",)
    // }
    // receiver

    // loop {
    //     match receiver.try_next() {
    //         Ok(msg) => {
    //             if let Some(msg) = msg {
    //                 let msg: = msg.try_into()
    //                 println!("recv msg")
    //             }
    //         }
    //         Err(_) => {}
    //     }
    // }

    // let _ = sleep(Duration::from_secs(1)).await;

    Ok(())
}

// fn print_entry(entry: Vec<GenlCtrlAttrs>) {
//     let family_id = entry
//         .iter()
//         .find_map(|nla| {
//             if let GenlCtrlAttrs::FamilyId(id) = nla {
//                 Some(*id)
//             } else {
//                 None
//             }
//         })
//         .expect("Cannot find FamilyId attribute");
//     let family_name = entry
//         .iter()
//         .find_map(|nla| {
//             if let GenlCtrlAttrs::FamilyName(name) = nla {
//                 Some(name.as_str())
//             } else {
//                 None
//             }
//         })
//         .expect("Cannot find FamilyName attribute");
//     let version = entry
//         .iter()
//         .find_map(|nla| {
//             if let GenlCtrlAttrs::Version(ver) = nla {
//                 Some(*ver)
//             } else {
//                 None
//             }
//         })
//         .expect("Cannot find Version attribute");
//     let hdrsize = entry
//         .iter()
//         .find_map(|nla| {
//             if let GenlCtrlAttrs::HdrSize(hdr) = nla {
//                 Some(*hdr)
//             } else {
//                 None
//             }
//         })
//         .expect("Cannot find HdrSize attribute");

//     if hdrsize == 0 {
//         println!("0x{family_id:04x} {family_name} [Version {version}]");
//     } else {
//         println!(
//             "0x{family_id:04x} {family_name} [Version {version}] \
//             [Header {hdrsize} bytes]"
//         );
//     }
// }
