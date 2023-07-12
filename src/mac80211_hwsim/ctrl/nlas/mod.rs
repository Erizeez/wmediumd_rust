use std::mem::size_of_val;

use anyhow::Context;
use bytemuck::cast_slice;
use netlink_packet_utils::{
    byteorder::{ByteOrder, NativeEndian},
    nla::{Nla, NlaBuffer},
    parsers::{parse_i32, parse_mac, parse_string, parse_u16, parse_u32, parse_u64},
    DecodeError, Emitable, Parseable,
};

use crate::mac80211_hwsim::{
    structs::{Frame, IEEE80211Header, ReceiverInfo, TXInfo, TXInfoFlag},
    MACAddress,
};

use super::super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HwsimAttrs {
    Unspec(),
    AddrReceiver(MACAddress),
    AddrTransmitter(MACAddress),
    Frame(Frame),
    Flags(u32),
    RXRate(u32),
    Signal(u32),
    TXInfo([TXInfo; IEEE80211_TX_MAX_RATES]),
    Cookie(u64),
    Channels(u32),
    RadioID(u32),
    RegHintAlpha2(String),
    RegCustomReg(u32),
    RegStrictReg(bool),
    SupportP2PDevice(bool),
    UseChanctx(bool),
    DestroyRadioOnClose(bool),
    RadioName(String),
    NoVif(bool),
    Freq(u32),
    Pad(),
    TXInfoFlags([TXInfoFlag; IEEE80211_TX_MAX_RATES]),
    PermAddr(MACAddress),
    IftypeSupport(u32),
    CipherSupport(Vec<u32>),
    SharedMemoryPointer(u64),
    SharedMemoryPageNum(u32),
}

impl Nla for HwsimAttrs {
    fn value_len(&self) -> usize {
        use HwsimAttrs::*;
        match self {
            Unspec() => todo!(),
            AddrReceiver(v) => size_of_val(v),
            AddrTransmitter(v) => size_of_val(v),
            Frame(v) => v.buffer_len(),
            Flags(v) => size_of_val(v),
            RXRate(v) => size_of_val(v),
            Signal(v) => size_of_val(v),
            TXInfo(v) => v[0].buffer_len() * IEEE80211_TX_MAX_RATES,
            Cookie(v) => size_of_val(v),
            Channels(v) => size_of_val(v),
            RadioID(v) => size_of_val(v),
            RegHintAlpha2(v) => size_of_val(v),
            RegCustomReg(v) => size_of_val(v),
            RegStrictReg(v) => 0,
            SupportP2PDevice(v) => 0,
            UseChanctx(v) => 0,
            DestroyRadioOnClose(v) => 0,
            RadioName(v) => (*v).as_bytes().len(),
            NoVif(v) => 0,
            Freq(v) => size_of_val(v),
            Pad() => todo!(),
            TXInfoFlags(_) => 0,
            PermAddr(v) => size_of_val(v),
            IftypeSupport(v) => size_of_val(v),
            CipherSupport(v) => size_of_val(v),
            SharedMemoryPointer(v) => size_of_val(v),
            SharedMemoryPageNum(v) => size_of_val(v),
        }
    }

    fn kind(&self) -> u16 {
        use HwsimAttrs::*;
        match self {
            Unspec() => HWSIM_ATTR_UNSPEC,
            AddrReceiver(_) => HWSIM_ATTR_ADDR_RECEIVER,
            AddrTransmitter(_) => HWSIM_ATTR_ADDR_TRANSMITTER,
            Frame(_) => HWSIM_ATTR_FRAME,
            Flags(_) => HWSIM_ATTR_FLAGS,
            RXRate(_) => HWSIM_ATTR_RX_RATE,
            Signal(_) => HWSIM_ATTR_SIGNAL,
            TXInfo(_) => HWSIM_ATTR_TX_INFO,
            Cookie(_) => HWSIM_ATTR_COOKIE,
            Channels(_) => HWSIM_ATTR_CHANNELS,
            RadioID(_) => HWSIM_ATTR_RADIO_ID,
            RegHintAlpha2(_) => HWSIM_ATTR_REG_HINT_ALPHA2,
            RegCustomReg(_) => HWSIM_ATTR_REG_CUSTOM_REG,
            RegStrictReg(_) => HWSIM_ATTR_REG_STRICT_REG,
            SupportP2PDevice(_) => HWSIM_ATTR_SUPPORT_P2P_DEVICE,
            UseChanctx(_) => HWSIM_ATTR_USE_CHANCTX,
            DestroyRadioOnClose(_) => HWSIM_ATTR_DESTROY_RADIO_ON_CLOSE,
            RadioName(_) => HWSIM_ATTR_RADIO_NAME,
            NoVif(_) => HWSIM_ATTR_NO_VIF,
            Freq(_) => HWSIM_ATTR_FREQ,
            Pad() => HWSIM_ATTR_PAD,
            TXInfoFlags(_) => HWSIM_ATTR_TX_INFO_FLAGS,
            PermAddr(_) => HWSIM_ATTR_PERM_ADDR,
            IftypeSupport(_) => HWSIM_ATTR_IFTYPE_SUPPORT,
            CipherSupport(_) => HWSIM_ATTR_CIPHER_SUPPORT,
            SharedMemoryPointer(_) => HWSIM_ATTR_SM_POINTER,
            SharedMemoryPageNum(_) => HWSIM_ATTR_SM_PAGE_NUM,
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use HwsimAttrs::*;
        match self {
            Unspec() => todo!(),
            AddrReceiver(v) => {
                buffer.copy_from_slice(v);
            }
            AddrTransmitter(v) => {
                buffer.copy_from_slice(v);
            }
            Frame(v) => v.emit(buffer),
            Flags(v) => NativeEndian::write_u32(buffer, *v),
            RXRate(v) => NativeEndian::write_u32(buffer, *v),
            Signal(v) => NativeEndian::write_u32(buffer, *v),
            TXInfo(v) => {
                let mut offset = 0;
                for rate in v {
                    rate.emit(&mut buffer[offset..]);
                    offset += rate.buffer_len();
                }
            }
            Cookie(v) => NativeEndian::write_u64(buffer, *v),
            Channels(v) => NativeEndian::write_u32(buffer, *v),
            RadioID(v) => NativeEndian::write_u32(buffer, *v),
            RegHintAlpha2(v) => {
                buffer.copy_from_slice((*v).as_bytes());
            }
            RegCustomReg(v) => NativeEndian::write_u32(buffer, *v),
            RegStrictReg(v) => {}
            SupportP2PDevice(v) => {}
            UseChanctx(v) => {}
            DestroyRadioOnClose(v) => {}
            RadioName(v) => {
                buffer.copy_from_slice((*v).as_bytes());
            }
            NoVif(v) => {}
            Freq(v) => NativeEndian::write_u32(buffer, *v),
            Pad() => todo!(),
            TXInfoFlags(_) => {}
            PermAddr(v) => {
                buffer.copy_from_slice(v);
            }
            IftypeSupport(v) => NativeEndian::write_u32(buffer, *v),
            CipherSupport(v) => {
                for vv in v {
                    NativeEndian::write_u32(buffer, *vv);
                }
            }
            SharedMemoryPointer(v) => NativeEndian::write_u64(buffer, *v),
            SharedMemoryPageNum(v) => NativeEndian::write_u32(buffer, *v),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for HwsimAttrs {
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            HWSIM_ATTR_ADDR_RECEIVER => Self::AddrReceiver(
                parse_mac(payload).context("failed to parse HWSIM_ATTR_ADDR_RECEIVER")?,
            ),
            HWSIM_ATTR_ADDR_TRANSMITTER => Self::AddrTransmitter(
                parse_mac(payload).context("failed to parse HWSIM_ATTR_ADDR_TRANSMITTER")?,
            ),
            HWSIM_ATTR_FRAME => {
                let mut frame_header: IEEE80211Header = IEEE80211Header::default();
                frame_header.frame_control.copy_from_slice(&payload[..2]);
                frame_header.duration_id.copy_from_slice(&payload[2..4]);
                frame_header.addr1.copy_from_slice(&payload[4..10]);
                frame_header.addr2.copy_from_slice(&payload[10..16]);
                frame_header.addr3.copy_from_slice(&payload[16..22]);
                frame_header.seq_ctrl.copy_from_slice(&payload[22..24]);
                frame_header.addr4.copy_from_slice(&payload[24..30]);
                frame_header.qos.copy_from_slice(&payload[30..32]);

                Self::Frame(Frame {
                    header: frame_header,
                    payload: payload[32..].to_vec(),
                })
            }
            HWSIM_ATTR_FLAGS => {
                Self::Flags(parse_u32(payload).context("failed to parse HWSIM_ATTR_FLAGS")?)
            }
            HWSIM_ATTR_RX_RATE => {
                Self::RXRate(parse_u32(payload).context("failed to parse HWSIM_ATTR_RX_RATE")?)
            }
            HWSIM_ATTR_TX_INFO => {
                let mut tx_info: [TXInfo; IEEE80211_TX_MAX_RATES] =
                    [TXInfo::default(); IEEE80211_TX_MAX_RATES];
                let mut offset = 0;

                for i in 0..IEEE80211_TX_MAX_RATES {
                    tx_info[i].idx = payload[offset] as i8;
                    tx_info[i].count = payload[offset + 1];
                    offset += 2;
                }

                Self::TXInfo(tx_info)
            }
            HWSIM_ATTR_COOKIE => {
                Self::Cookie(parse_u64(payload).context("failed to parse HWSIM_ATTR_COOKIE")?)
            }
            HWSIM_ATTR_CHANNELS => {
                Self::Channels(parse_u32(payload).context("failed to parse HWSIM_ATTR_CHANNELS")?)
            }
            HWSIM_ATTR_RADIO_ID => {
                Self::RadioID(parse_u32(payload).context("failed to parse HWSIM_ATTR_RADIO_ID")?)
            }
            HWSIM_ATTR_REG_HINT_ALPHA2 => Self::RegHintAlpha2(
                parse_string(payload).context("failed to parse HWSIM_ATTR_REG_HINT_ALPHA2")?,
            ),
            HWSIM_ATTR_REG_CUSTOM_REG => Self::RegCustomReg(
                parse_u32(payload).context("failed to parse HWSIM_ATTR_REG_CUSTOM_REG")?,
            ),
            HWSIM_ATTR_REG_STRICT_REG => Self::RegStrictReg(true),
            HWSIM_ATTR_SUPPORT_P2P_DEVICE => Self::SupportP2PDevice(true),
            HWSIM_ATTR_USE_CHANCTX => Self::RegStrictReg(true),
            HWSIM_ATTR_RADIO_NAME => Self::RadioName(
                parse_string(payload).context("failed to parse HWSIM_ATTR_RADIO_NAME")?,
            ),
            HWSIM_ATTR_FREQ => {
                Self::Freq(parse_u32(payload).context("failed to parse HWSIM_ATTR_FREQ")?)
            }
            HWSIM_ATTR_TX_INFO_FLAGS => {
                let mut tx_info_flags: [TXInfoFlag; IEEE80211_TX_MAX_RATES] =
                    [TXInfoFlag::default(); IEEE80211_TX_MAX_RATES];
                let mut offset = 0;

                for i in 0..IEEE80211_TX_MAX_RATES {
                    tx_info_flags[i].idx = payload[offset] as i8;
                    tx_info_flags[i].flags = parse_u16(&payload[offset + 1..offset + 3])
                        .context("failed to parse HWSIM_ATTR_TX_INFO_FLAGS")?;
                    offset += 3;
                }

                Self::TXInfoFlags(tx_info_flags)
            }
            HWSIM_ATTR_PERM_ADDR => {
                Self::PermAddr(parse_mac(payload).context("failed to parse HWSIM_ATTR_PERM_ADDR")?)
            }
            HWSIM_ATTR_SM_POINTER => Self::SharedMemoryPointer(parse_u64(payload)?),
            HWSIM_ATTR_SM_PAGE_NUM => Self::SharedMemoryPageNum(parse_u32(payload)?),
            kind => {
                // println!("Unknown NLA type: {kind}");
                return Err(DecodeError::from(format!("Unknown NLA type: {kind}")));
            }
        })
    }
}
