use std::time::Duration;

use anyhow::{bail, Error};
use futures::{stream::Stream, StreamExt};
use genetlink::{
    self,
    message::{map_from_rawgenlmsg, map_to_rawgenlmsg},
    new_connection,
};
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NLM_F_REQUEST};
use netlink_packet_generic::{ctrl::nlas::GenlCtrlAttrs, GenlMessage};
use netlink_packet_utils::{nla::Nla, ParseableParametrized};
use tokio::time::sleep;

use crate::mac80211_hwsim::{
    ctrl::nlas::HwsimAttrs,
    structs::{ReceiverInfo, TXInfo},
};

use self::mac80211_hwsim::ctrl::*;

use netlink_packet_core::NetlinkDeserializable;

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

    let nlmsg = NetlinkMessage::new(
        nl_hdr,
        GenlMessage::from_payload(GenlMAC {
            cmd: HwsimCmd::Register,
            nlas: vec![],
        })
        .into(),
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

                        let mut nlas = vec![];
                        let mut receiver_info = ReceiverInfo::default();

                        for attr in &frame.nlas {
                            match attr {
                                HwsimAttrs::AddrTransmitter(v) => {
                                    if v.eq(&[0x42, 0x0, 0x0, 0x0, 0x1, 0x0]) {
                                        receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x0, 0x0];
                                    } else {
                                        receiver_info.addr = [0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
                                    }
                                    nlas.push(HwsimAttrs::AddrTransmitter(v.clone()));
                                }
                                HwsimAttrs::Flags(v) => nlas.push(HwsimAttrs::Flags(*v)),
                                HwsimAttrs::TXInfo(v) => {
                                    nlas.push(HwsimAttrs::RXRate(v.tx_rates[0].idx as u32));
                                    let info = TXInfo::default();
                                    // nlas.push(HwsimAttrs::TXInfo(v.clone()));
                                    nlas.push(HwsimAttrs::TXInfo(info));
                                }
                                HwsimAttrs::Cookie(v) => nlas.push(HwsimAttrs::Cookie(*v)),
                                HwsimAttrs::Freq(v) => {
                                    nlas.push(HwsimAttrs::Freq(*v));
                                }
                                _ => {}
                            }
                        }

                        let signal = (30 - 91) as u32;
                        receiver_info.signal = signal;
                        nlas.push(HwsimAttrs::ReceiverInfo(receiver_info));
                        nlas.push(HwsimAttrs::Signal(signal));
                        dbg!(&nlas);

                        let mut nl_hdr = NetlinkHeader::default();
                        nl_hdr.flags = NLM_F_REQUEST;

                        let mut nlmsg = NetlinkMessage::new(
                            nl_hdr,
                            GenlMessage::from_payload(GenlMAC {
                                cmd: HwsimCmd::YawmdRXInfo,
                                nlas,
                            })
                            .into(),
                        );
                        print!("{}", nlmsg.buffer_len());
                        nlmsg.finalize();
                        let mut buffer = vec![0; nlmsg.buffer_len()];
                        nlmsg.serialize(&mut buffer);
                        print!("{:?}", buffer);
                        // println!("send");
                        match handle.request(nlmsg).await {
                            Ok(_) => {}
                            Err(_) => {
                                println!("error")
                            }
                        }
                        println!("sended");
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

fn print_entry(entry: Vec<GenlCtrlAttrs>) {
    let family_id = entry
        .iter()
        .find_map(|nla| {
            if let GenlCtrlAttrs::FamilyId(id) = nla {
                Some(*id)
            } else {
                None
            }
        })
        .expect("Cannot find FamilyId attribute");
    let family_name = entry
        .iter()
        .find_map(|nla| {
            if let GenlCtrlAttrs::FamilyName(name) = nla {
                Some(name.as_str())
            } else {
                None
            }
        })
        .expect("Cannot find FamilyName attribute");
    let version = entry
        .iter()
        .find_map(|nla| {
            if let GenlCtrlAttrs::Version(ver) = nla {
                Some(*ver)
            } else {
                None
            }
        })
        .expect("Cannot find Version attribute");
    let hdrsize = entry
        .iter()
        .find_map(|nla| {
            if let GenlCtrlAttrs::HdrSize(hdr) = nla {
                Some(*hdr)
            } else {
                None
            }
        })
        .expect("Cannot find HdrSize attribute");

    if hdrsize == 0 {
        println!("0x{family_id:04x} {family_name} [Version {version}]");
    } else {
        println!(
            "0x{family_id:04x} {family_name} [Version {version}] \
            [Header {hdrsize} bytes]"
        );
    }
}
