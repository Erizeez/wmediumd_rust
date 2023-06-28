use anyhow::Error;
use futures::StreamExt;
use genetlink::{self, new_connection};
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;
use netlink_packet_utils::ParseableParametrized;

use crate::mac80211_hwsim::{
    constants::MICROSECONDS_TO_NANOSECONDS, ctrl::nlas::HwsimAttrs, structs::ReceiverInfo,
};

use self::mac80211_hwsim::ctrl::*;

mod mac80211_hwsim;

#[tokio::main]
async fn main() -> Result<(), Error> {
    init_genetlink().await?;
    Ok(())
}

async fn init_genetlink() -> Result<(), Error> {
    println!("Start Genetlink");
    let mut nl_hdr = NetlinkHeader::default();
    nl_hdr.flags = NLM_F_REQUEST;

    let nlmsg: NetlinkMessage<GenlMessage<_>> = NetlinkMessage::new(
        nl_hdr,
        NetlinkPayload::InnerMessage(GenlMessage::from_payload(GenlMAC {
            cmd: HwsimCmd::Register,
            nlas: vec![],
        })),
    );
    let (conn, mut handle, mut receiver) = new_connection()?;
    tokio::spawn(conn);

    handle.notify(nlmsg).await?;

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

                        // println!("receive");
                        // frame = dbg!(frame);
                        // frame.nlas.iter_map().for_each(|e| match e {
                        //     HwsimAttrs::TXInfo(v) => frame
                        //         .nlas
                        //         .push(HwsimAttrs::RXRate(v.tx_rates[0].idx.try_into().unwrap())),
                        //     _ => {}
                        // });

                        let mut new_nlas = vec![];
                        let mut receiver_info = ReceiverInfo::default();

                        for attr in &frame.nlas {
                            match attr {
                                HwsimAttrs::AddrTransmitter(v) => {
                                    if v.eq(&[0x42, 0x0, 0x0, 0x0, 0x1, 0x0]) {
                                        receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x0, 0x0];
                                    } else {
                                        receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
                                    }
                                    new_nlas.push(HwsimAttrs::AddrTransmitter(v.clone()));
                                }
                                HwsimAttrs::Flags(v) => new_nlas.push(HwsimAttrs::Flags(*v)),
                                HwsimAttrs::TXInfo(v) => {
                                    new_nlas.push(HwsimAttrs::RXRate(v.tx_rates[0].idx as u32));

                                    new_nlas.push(HwsimAttrs::TXInfo(v.clone()));
                                }
                                HwsimAttrs::Cookie(v) => new_nlas.push(HwsimAttrs::Cookie(*v)),
                                HwsimAttrs::Freq(v) => {
                                    new_nlas.push(HwsimAttrs::Freq(*v));
                                }
                                HwsimAttrs::TimeStamp(v) => {
                                    new_nlas.push(HwsimAttrs::TimeStamp(
                                        *v + 1000 * MICROSECONDS_TO_NANOSECONDS,
                                    ));
                                    println!("{}", &v);
                                }
                                _ => {}
                            }
                        }

                        let signal = (30 - 91) as u32;
                        receiver_info.signal = signal;
                        new_nlas.push(HwsimAttrs::ReceiverInfo(receiver_info));
                        new_nlas.push(HwsimAttrs::Signal(signal));

                        // dbg!(&nlas);

                        let mut nl_hdr = NetlinkHeader::default();
                        nl_hdr.flags = NLM_F_REQUEST;

                        let mut nlmsg = NetlinkMessage::new(
                            nl_hdr,
                            GenlMessage::from_payload(GenlMAC {
                                cmd: HwsimCmd::YawmdRXInfo,
                                nlas: new_nlas,
                            })
                            .into(),
                        );

                        match handle.notify(nlmsg).await {
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
