use anyhow::{bail, Error};
use futures::StreamExt;
use genetlink::{self, new_connection};
use netlink_packet_core::{
    NetlinkHeader, NetlinkMessage, NetlinkPayload, NLM_F_ACK, NLM_F_DUMP, NLM_F_REQUEST,
};
use netlink_packet_generic::{
    ctrl::{nlas::GenlCtrlAttrs, GenlCtrl, GenlCtrlCmd},
    GenlFamily, GenlMessage,
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

    let nlmsg = NetlinkMessage::new(
        nl_hdr,
        GenlMessage::from_payload(GenlMAC {
            cmd: HwsimCmd::Register.into(),
            nlas: vec![],
        })
        .into(),
    );
    let (conn, mut handle, _) = new_connection()?;
    tokio::spawn(conn);

    handle.resolve_family_id::<GenlMAC>();

    handle.notify(nlmsg).await?;

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
