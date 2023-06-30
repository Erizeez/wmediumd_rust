use anyhow::Error;
use futures::StreamExt;
use genetlink::{self, new_connection};
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;
use netlink_packet_utils::ParseableParametrized;

use crate::{
    mac80211_hwsim::{
        constants::MICROSECONDS_TO_NANOSECONDS, ctrl::nlas::HwsimAttrs, structs::ReceiverInfo,
    },
    structs::{GenlRegister, GenlYawmdRXInfo, GenlYawmdTXInfo},
};

use self::mac80211_hwsim::ctrl::*;

mod mac80211_hwsim;
mod structs;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_genetlink().await?;
    Ok(())
}

async fn init_genetlink() -> Result<(), Error> {
    println!("Start Genetlink");

    let genl_register = GenlRegister {};

    let (conn, mut handle, mut receiver) = new_connection()?;
    tokio::spawn(conn);

    handle.notify(genl_register.generate_genl_message()).await?;

    while let Some(msg) = receiver.next().await {
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

                        let mut receiver_info = ReceiverInfo::default();
                        if data.addr_transmitter.eq(&[0x42, 0x0, 0x0, 0x0, 0x1, 0x0]) {
                            receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x0, 0x0];
                        } else {
                            receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
                        }
                        receiver_info.signal = signal;
                        rx_info.receiver_info = receiver_info;

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
