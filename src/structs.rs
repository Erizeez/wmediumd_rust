use crate::mac80211_hwsim::constants::*;
use crate::mac80211_hwsim::ctrl::nlas::HwsimAttrs;
use crate::mac80211_hwsim::ctrl::{GenlAutoConstruct, GenlMAC, HwsimCmd};
use crate::mac80211_hwsim::structs::{IEEE80211Header, ReceiverInfo, TXInfo, TXInfoFlag};
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NLM_F_REQUEST};
use netlink_packet_generic::GenlMessage;

pub struct GenlRegister {}

impl GenlAutoConstruct for GenlRegister {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::Register,
                nlas: vec![],
            })
            .into(),
        )
    }

    fn parse(_: GenlMAC) -> Self {
        panic!("parse function not implemented for GenlRegister");
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlYawmdTXInfo {
    pub addr_transmitter: [u8; ETH_ALEN],
    pub flags: u32,
    pub tx_info: [TXInfo; IEEE80211_TX_MAX_RATES],
    pub cookie: u64,
    pub freq: u32,
    pub tx_info_flags: [TXInfoFlag; IEEE80211_TX_MAX_RATES],
    pub frame_header: IEEE80211Header,
    pub frame_length: u32,
    pub timestamp: i64,
}

impl GenlAutoConstruct for GenlYawmdTXInfo {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        panic!("generate_genl_message function not implemented for GenlRegister");
    }

    fn parse(data: GenlMAC) -> Self {
        let mut parsed_data = GenlYawmdTXInfo::default();
        for attr in &data.nlas {
            use HwsimAttrs::*;
            match attr {
                AddrTransmitter(v) => {
                    parsed_data.addr_transmitter = *v;
                }
                Flags(v) => {
                    parsed_data.flags = *v;
                }
                TXInfo(v) => {
                    parsed_data.tx_info = *v;
                }
                Cookie(v) => {
                    parsed_data.cookie = *v;
                }
                Freq(v) => {
                    parsed_data.freq = *v;
                }
                TXInfoFlags(v) => {
                    parsed_data.tx_info_flags = *v;
                }
                FrameHeader(v) => {
                    parsed_data.frame_header = (*v).clone();
                }
                FrameLength(v) => {
                    parsed_data.frame_length = *v;
                }
                TimeStamp(v) => {
                    parsed_data.timestamp = *v;
                }
                _ => {}
            }
        }
        parsed_data
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlYawmdRXInfo {
    pub addr_transmitter: [u8; ETH_ALEN],
    pub flags: u32,
    pub rx_rate: u32,
    pub signal: u32,
    pub tx_info: [TXInfo; IEEE80211_TX_MAX_RATES],
    pub cookie: u64,
    pub freq: u32,
    pub timestamp: i64,
    pub receiver_info: ReceiverInfo,
}

impl GenlAutoConstruct for GenlYawmdRXInfo {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        let mut nlas = vec![];

        use HwsimAttrs::*;

        nlas.push(AddrTransmitter(self.addr_transmitter));
        nlas.push(Flags(self.signal));
        nlas.push(RXRate(self.rx_rate));
        nlas.push(Signal(self.signal));
        nlas.push(TXInfo(self.tx_info));
        nlas.push(Cookie(self.cookie));
        nlas.push(Freq(self.freq));
        nlas.push(TimeStamp(self.timestamp));
        nlas.push(ReceiverInfo(self.receiver_info.clone()));

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::YawmdRXInfo,
                nlas,
            })
            .into(),
        )
    }

    fn parse(data: GenlMAC) -> Self {
        panic!("parse function not implemented for GenlYawmdRXInfo");
    }
}
