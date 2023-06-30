use std::mem::size_of_val;

use anyhow::Context;
use netlink_packet_utils::{
    byteorder::{ByteOrder, NativeEndian},
    nla::{Nla, NlaBuffer},
    parsers::{parse_mac, parse_u16, parse_u32, parse_u64},
    DecodeError, Emitable, Parseable,
};

use crate::mac80211_hwsim::structs::{IEEE80211Header, ReceiverInfo, TXInfo, TXInfoFlag};

use super::super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HwsimAttrs {
    Unspec(),
    AddrReceiver(),
    AddrTransmitter([u8; ETH_ALEN]),
    Frame(),
    Flags(u32),
    RXRate(u32),
    Signal(u32),
    TXInfo([TXInfo; IEEE80211_TX_MAX_RATES]),
    Cookie(u64),
    Channels(u32),
    RadioID(),
    RegHintAlpha2(),
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
    PermAddr(),
    IftypeSupport(u32),
    CipherSupport(),
    FrameHeader(IEEE80211Header),
    FrameLength(u32),
    FrameID(),
    ReceiverInfo(ReceiverInfo),
    TimeStamp(i64),
}

impl Nla for HwsimAttrs {
    fn value_len(&self) -> usize {
        use HwsimAttrs::*;
        match self {
            Unspec() => todo!(),
            AddrReceiver() => todo!(),
            AddrTransmitter(v) => size_of_val(v),
            Frame() => todo!(),
            Flags(v) => size_of_val(v),
            RXRate(v) => size_of_val(v),
            Signal(v) => size_of_val(v),
            TXInfo(v) => v[0].buffer_len() * IEEE80211_TX_MAX_RATES,
            Cookie(v) => size_of_val(v),
            Channels(v) => todo!(),
            RadioID() => todo!(),
            RegHintAlpha2() => todo!(),
            RegCustomReg(v) => todo!(),
            RegStrictReg(v) => todo!(),
            SupportP2PDevice(v) => todo!(),
            UseChanctx(v) => todo!(),
            DestroyRadioOnClose(v) => todo!(),
            RadioName(v) => todo!(),
            NoVif(v) => todo!(),
            Freq(v) => size_of_val(v),
            Pad() => todo!(),
            TXInfoFlags(_) => 0,
            PermAddr() => todo!(),
            IftypeSupport(v) => todo!(),
            CipherSupport() => todo!(),
            FrameHeader(_) => 0,
            FrameLength(_) => 0,
            FrameID() => todo!(),
            ReceiverInfo(v) => v.buffer_len(),
            TimeStamp(v) => size_of_val(v),
        }
    }

    fn kind(&self) -> u16 {
        use HwsimAttrs::*;
        match self {
            Unspec() => HWSIM_ATTR_UNSPEC,
            AddrReceiver() => HWSIM_ATTR_ADDR_RECEIVER,
            AddrTransmitter(_) => HWSIM_ATTR_ADDR_TRANSMITTER,
            Frame() => HWSIM_ATTR_FRAME,
            Flags(_) => HWSIM_ATTR_FLAGS,
            RXRate(_) => HWSIM_ATTR_RX_RATE,
            Signal(_) => HWSIM_ATTR_SIGNAL,
            TXInfo(_) => HWSIM_ATTR_TX_INFO,
            Cookie(_) => HWSIM_ATTR_COOKIE,
            Channels(_) => HWSIM_ATTR_CHANNELS,
            RadioID() => HWSIM_ATTR_RADIO_ID,
            RegHintAlpha2() => HWSIM_ATTR_REG_HINT_ALPHA2,
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
            PermAddr() => HWSIM_ATTR_PERM_ADDR,
            IftypeSupport(_) => HWSIM_ATTR_IFTYPE_SUPPORT,
            CipherSupport() => HWSIM_ATTR_CIPHER_SUPPORT,
            FrameHeader(_) => HWSIM_ATTR_FRAME_HEADER,
            FrameLength(_) => HWSIM_ATTR_FRAME_LENGTH,
            FrameID() => HWSIM_ATTR_FRAME_ID,
            ReceiverInfo(_) => HWSIM_ATTR_RECEIVER_INFO,
            TimeStamp(_) => HWSIM_ATTR_FRAME_TIMESTAMP,
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use HwsimAttrs::*;
        match self {
            Unspec() => todo!(),
            AddrReceiver() => todo!(),
            AddrTransmitter(v) => {
                buffer.copy_from_slice(v);
            }
            Frame() => todo!(),
            Flags(v) => NativeEndian::write_u32(buffer, *v),
            RXRate(v) => NativeEndian::write_u32(buffer, *v),
            Signal(v) => NativeEndian::write_u32(buffer, *v),
            TXInfo(v) => {
                // println!("start emit");
                // dbg!(v);
                // v.emit(buffer);
                // println!("end emit");
            }
            Cookie(v) => NativeEndian::write_u64(buffer, *v),
            Channels(v) => todo!(),
            RadioID() => todo!(),
            RegHintAlpha2() => todo!(),
            RegCustomReg(v) => todo!(),
            RegStrictReg(v) => todo!(),
            SupportP2PDevice(v) => todo!(),
            UseChanctx(v) => todo!(),
            DestroyRadioOnClose(v) => todo!(),
            RadioName(v) => todo!(),
            NoVif(v) => todo!(),
            Freq(v) => NativeEndian::write_u32(buffer, *v),
            Pad() => todo!(),
            TXInfoFlags(_) => {}
            PermAddr() => todo!(),
            IftypeSupport(v) => todo!(),
            CipherSupport() => todo!(),
            FrameHeader(_) => {}
            FrameLength(_) => {}
            FrameID() => todo!(),
            ReceiverInfo(v) => (*v).emit(buffer),
            TimeStamp(v) => NativeEndian::write_i64(buffer, *v),
        }
    }
}

impl<'a, T: AsRef<[u8]> + ?Sized> Parseable<NlaBuffer<&'a T>> for HwsimAttrs {
    fn parse(buf: &NlaBuffer<&'a T>) -> Result<Self, DecodeError> {
        let payload = buf.value();
        Ok(match buf.kind() {
            HWSIM_ATTR_ADDR_TRANSMITTER => Self::AddrTransmitter(
                parse_mac(payload).context("failed to parse HWSIM_ATTR_ADDR_TRANSMITTER")?,
            ),
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
            HWSIM_ATTR_FRAME_HEADER => {
                let mut frame_header: IEEE80211Header = IEEE80211Header::default();
                frame_header.frame_control.copy_from_slice(&payload[..2]);
                frame_header.duration_id.copy_from_slice(&payload[2..4]);
                frame_header.addr1.copy_from_slice(&payload[4..10]);
                frame_header.addr2.copy_from_slice(&payload[10..16]);
                frame_header.addr3.copy_from_slice(&payload[16..22]);
                frame_header.seq_ctrl.copy_from_slice(&payload[22..24]);
                frame_header.addr4.copy_from_slice(&payload[24..30]);
                frame_header.qos.copy_from_slice(&payload[30..32]);

                Self::FrameHeader(frame_header)
            }
            HWSIM_ATTR_FRAME_LENGTH => Self::FrameLength(
                parse_u32(payload).context("failed to parse HWSIM_ATTR_FRAME_LENGTH")?,
            ),
            HWSIM_ATTR_FRAME_TIMESTAMP => Self::TimeStamp(
                parse_u64(payload)
                    .context("ailed to parse HWSIM_ATTR_FRAME_TIMESTAMP")?
                    .try_into()
                    .unwrap(),
            ),
            kind => return Err(DecodeError::from(format!("Unknown NLA type: {kind}"))),
        })
    }
}
