use crate::mac80211_hwsim::ctrl::nlas::HwsimAttrs;
use crate::mac80211_hwsim::ctrl::{GenlAutoConstruct, GenlMAC, HwsimCmd};
use crate::mac80211_hwsim::structs::{Frame, IEEE80211Header, ReceiverInfo, TXInfo, TXInfoFlag};
use crate::mac80211_hwsim::{constants::*, MACAddress};
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NLM_F_DUMP, NLM_F_REQUEST};
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
pub struct GenlFrameTX {
    pub addr_transmitter: MACAddress,
    pub frame: Frame,
    pub flags: u32,
    pub tx_info: [TXInfo; IEEE80211_TX_MAX_RATES],
    pub cookie: u64,
    pub freq: u32,
    pub tx_info_flags: [TXInfoFlag; IEEE80211_TX_MAX_RATES],
    pub is_ack: bool,
}

impl GenlAutoConstruct for GenlFrameTX {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        panic!("generate_genl_message function not implemented for GenlRegister");
    }

    fn parse(data: GenlMAC) -> Self {
        let mut parsed_data = GenlFrameTX::default();
        for attr in &data.nlas {
            use HwsimAttrs::*;
            match attr {
                AddrTransmitter(v) => {
                    parsed_data.addr_transmitter = *v;
                }
                Frame(v) => {
                    parsed_data.frame = v.clone();
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
                _ => {}
            }
        }
        parsed_data
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlFrameRX {
    pub addr_receiver: MACAddress,
    pub frame: Frame,
    pub rx_rate: u32,
    pub signal: u32,
    pub freq: u32,
}

impl GenlAutoConstruct for GenlFrameRX {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        let mut nlas = vec![];

        use HwsimAttrs::*;

        nlas.push(AddrReceiver(self.addr_receiver));
        nlas.push(Frame(self.frame.clone()));
        nlas.push(RXRate(self.rx_rate));
        nlas.push(Signal(self.signal));
        nlas.push(Freq(self.freq));

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::Frame,
                nlas,
            })
            .into(),
        )
    }

    fn parse(data: GenlMAC) -> Self {
        panic!("parse function not implemented for GenlYawmdRXInfo");
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlTXInfoFrame {
    pub addr_transmitter: MACAddress,
    pub flags: u32,
    pub cookie: u64,
    pub signal: u32,
    pub tx_info: [TXInfo; IEEE80211_TX_MAX_RATES],
}

impl GenlAutoConstruct for GenlTXInfoFrame {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        let mut nlas = vec![];

        use HwsimAttrs::*;

        nlas.push(AddrTransmitter(self.addr_transmitter));
        nlas.push(Flags(self.flags));
        nlas.push(Cookie(self.cookie));
        nlas.push(Signal(self.signal));
        nlas.push(TXInfo(self.tx_info));

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::TXInfoFrame,
                nlas,
            })
            .into(),
        )
    }

    fn parse(data: GenlMAC) -> Self {
        panic!("parse function not implemented for GenlYawmdRXInfo");
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlNewRadio {
    pub channels: u32,
    pub reg_hint_alpha2: String,
    pub reg_custom_reg: u32,
    pub reg_strict_reg: bool,
    pub support_p2p_device: bool,
    pub use_chanctx: bool,
    pub destroy_radio_on_close: bool,
    pub radio_name: String,
    pub no_vif: bool,
    pub perm_addr: [u8; ETH_ALEN],
    pub iftype_support: u32,
    pub cipher_support: Vec<u32>,
}

impl GenlAutoConstruct for GenlNewRadio {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        let mut nlas = vec![];

        use HwsimAttrs::*;

        nlas.push(Channels(self.channels));
        // nlas.push(RegHintAlpha2(self.reg_hint_alpha2.clone()));
        // nlas.push(RegCustomReg(self.reg_custom_reg));
        if self.support_p2p_device {
            nlas.push(SupportP2PDevice(self.support_p2p_device));
        }
        if self.use_chanctx {
            nlas.push(UseChanctx(self.use_chanctx));
        }
        if self.destroy_radio_on_close {
            nlas.push(DestroyRadioOnClose(self.destroy_radio_on_close));
        }
        nlas.push(RadioName(self.radio_name.clone()));
        if self.no_vif {
            nlas.push(NoVif(self.no_vif));
        }
        nlas.push(PermAddr(self.perm_addr));
        // nlas.push(IftypeSupport(self.iftype_support));
        // nlas.push(CipherSupport(self.cipher_support.clone()));

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::NewRadio,
                nlas,
            })
            .into(),
        )
    }

    fn parse(data: GenlMAC) -> Self {
        panic!("parse function not implemented for GenlNewRadio");
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct GenlRadio {
    pub channels: u32,
    pub reg_hint_alpha2: String,
    pub reg_custom_reg: u32,
    pub reg_strict_reg: bool,
    pub support_p2p_device: bool,
    pub use_chanctx: bool,
    pub destroy_radio_on_close: bool,
    pub radio_name: String,
    pub no_vif: bool,
    pub perm_addr: [u8; ETH_ALEN],
    pub iftype_support: u32,
    pub cipher_support: Vec<u32>,
}

#[derive(Default)]
pub struct GenlRadioOps {
    pub idx: u32,
}

impl GenlRadioOps {
    pub fn generate_genl_dump(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST | NLM_F_DUMP;

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::GetRadio,
                nlas: vec![],
            })
            .into(),
        )
    }

    pub fn generate_genl_get(&self) -> NetlinkMessage<GenlMessage<GenlMAC>> {
        let mut nl_hdr = NetlinkHeader::default();
        nl_hdr.flags = NLM_F_REQUEST;

        let mut nlas = vec![];

        use HwsimAttrs::*;

        nlas.push(RadioID(self.idx));

        NetlinkMessage::new(
            nl_hdr,
            GenlMessage::from_payload(GenlMAC {
                cmd: HwsimCmd::GetRadio,
                nlas: nlas,
            })
            .into(),
        )
    }

    // pub fn parse_dump(data: GenlMAC) -> Self {}
}
