use std::{
    io::{self},
    mem,
    os::fd::AsRawFd,
    task::{Context, Poll},
};

use futures::StreamExt;
use genetlink::{self, new_connection};
use libc::{setsockopt, SOL_SOCKET, SO_RCVBUF, SO_SNDBUF};
use netlink_packet_utils::ParseableParametrized;
use netlink_sys::{AsyncSocket, Socket, SocketAddr};
use structs::{GenlFrameRX, GenlFrameTX, GenlRegister, GenlTXInfoFrame};

use self::mac80211_hwsim::ctrl::*;

mod config;
mod mac80211_hwsim;
mod mysocket;
mod structs;

#[tokio::main]
async fn main() {
    radio_process().await;
}

async fn radio_process() {
    let (mut conn, mut handle, mut receiver) =
        new_connection().expect("Failed to create connection.");

    let raw_fd = conn.socket_mut().as_raw_fd();
    let buffer_len = 1024 * 1024 * 1;
    tokio::spawn(conn);

    unsafe {
        let payload = &buffer_len as *const i32 as *const libc::c_void;
        let payload_len = mem::size_of::<i32>() as libc::socklen_t;
        setsockopt(raw_fd, SOL_SOCKET, SO_SNDBUF, payload, payload_len);
        setsockopt(raw_fd, SOL_SOCKET, SO_RCVBUF, payload, payload_len);
    }

    // Register wmediumd using genetlink
    let genl_register = GenlRegister {};
    handle
        .notify(genl_register.generate_genl_message())
        .await
        .expect("Failed to register wmediumd");

    println!("start radio process");
    loop {
        let msg = if let Some(msg) = receiver.next().await {
            msg.0
        } else {
            continue;
        };

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

                        let signal = (30 - 91) as u32;

                        let mut tx_info_frame = GenlTXInfoFrame::default();
                        tx_info_frame.addr_transmitter = data.addr_transmitter;
                        tx_info_frame.flags = data.flags;
                        tx_info_frame.tx_info = data.tx_info;
                        tx_info_frame.cookie = data.cookie;

                        let mut frame_rx = GenlFrameRX::default();
                        frame_rx.rx_rate = data.tx_info[0].idx as u32;
                        frame_rx.signal = signal;
                        frame_rx.freq = data.freq;
                        frame_rx.frame = data.frame;

                        if data.addr_transmitter.eq(&[0x42, 0x0, 0x0, 0x0, 0x1, 0x0]) {
                            frame_rx.addr_receiver = [0x42, 0x0, 0x0, 0x0, 0x0, 0x0];
                        } else {
                            frame_rx.addr_receiver = [0x42, 0x0, 0x0, 0x0, 0x1, 0x0];
                        }

                        match handle.notify(frame_rx.generate_genl_message()).await {
                            Ok(_) => {
                                // println!("handle 1 frame rx");
                            }
                            Err(_) => {
                                println!("fail frame rx");
                                // panic!("fail frame rx");
                            }
                        }

                        // match handle.notify(tx_info_frame.generate_genl_message()).await {
                        //     Ok(_) => {
                        //         // println!("handle 1 frame tx info");
                        //     }
                        //     Err(_) => {
                        //         println!("fail frame tx info");
                        //         panic!("fail frame tx info");
                        //     }
                        // }
                    }
                    Err(_) => {}
                }
            }
            _ => todo!(),
        }
    }
}
