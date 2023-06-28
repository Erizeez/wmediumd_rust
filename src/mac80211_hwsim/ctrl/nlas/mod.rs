use std::mem::size_of_val;

use anyhow::Context;
use netlink_packet_utils::{
    byteorder::{ByteOrder, NativeEndian},
    nla::{Nla, NlaBuffer},
    parsers::{parse_mac, parse_u32, parse_u64},
    DecodeError, Emitable, Parseable,
};

use crate::mac80211_hwsim::structs::{IEEE80211Header, ReceiverInfo, TXInfo, TXRate};

use super::super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HwsimAttrs {
    AddrTransmitter([u8; 6]),
    Flags(u32),
    RXRate(u32),
    Signal(u32),
    TXInfo(TXInfo),
    Cookie(u64),
    Freq(u32),
    TXInfoFlags(u16),
    FrameHeader(IEEE80211Header),
    FrameLength(u32),
    ReceiverInfo(ReceiverInfo),
    TimeStamp(i64),
}

impl Nla for HwsimAttrs {
    fn value_len(&self) -> usize {
        use HwsimAttrs::*;
        match self {
            AddrTransmitter(v) => size_of_val(v),
            Flags(v) => size_of_val(v),
            RXRate(v) => size_of_val(v),
            Signal(v) => size_of_val(v),
            TXInfo(v) => v.buffer_len(),
            Cookie(v) => size_of_val(v),
            Freq(v) => size_of_val(v),
            TXInfoFlags(_) => 0,
            FrameHeader(_) => 0,
            FrameLength(_) => 0,
            ReceiverInfo(v) => v.buffer_len(),
            TimeStamp(v) => size_of_val(v),
        }
    }

    fn kind(&self) -> u16 {
        use HwsimAttrs::*;
        match self {
            AddrTransmitter(_) => HWSIM_ATTR_ADDR_TRANSMITTER,
            Flags(_) => HWSIM_ATTR_FLAGS,
            RXRate(_) => HWSIM_ATTR_RX_RATE,
            Signal(_) => HWSIM_ATTR_SIGNAL,
            TXInfo(_) => HWSIM_ATTR_TX_INFO,
            Cookie(_) => HWSIM_ATTR_COOKIE,
            Freq(_) => HWSIM_ATTR_FREQ,
            TXInfoFlags(_) => HWSIM_ATTR_TX_INFO_FLAGS,
            FrameHeader(_) => HWSIM_ATTR_FRAME_HEADER,
            FrameLength(_) => HWSIM_ATTR_FRAME_LENGTH,
            ReceiverInfo(_) => HWSIM_ATTR_RECEIVER_INFO,
            TimeStamp(_) => HWSIM_ATTR_FRAME_TIMESTAMP,
        }
    }

    fn emit_value(&self, buffer: &mut [u8]) {
        use HwsimAttrs::*;
        match self {
            AddrTransmitter(v) => {
                buffer.copy_from_slice(v);
            }
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
            Freq(v) => NativeEndian::write_u32(buffer, *v),
            TXInfoFlags(_) => {}
            FrameHeader(_) => {}
            FrameLength(_) => {}
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
                let mut tx_info: TXInfo = TXInfo::default();
                let tx_rates_len = payload.len();
                tx_info.tx_rates_count = tx_rates_len as i32 / 2;
                for i in 0..tx_rates_len {
                    if i >= 8 {
                        break;
                    }
                    if i % 2 == 0 {
                        tx_info.tx_rates[i / 2].idx = payload[i] as i8;
                    } else {
                        tx_info.tx_rates[i / 2].count = payload[i];
                    }
                }

                Self::TXInfo(tx_info)
            }
            HWSIM_ATTR_COOKIE => {
                Self::Cookie(parse_u64(payload).context("failed to parse HWSIM_ATTR_COOKIE")?)
            }
            HWSIM_ATTR_FREQ => {
                Self::Freq(parse_u32(payload).context("failed to parse HWSIM_ATTR_FREQ")?)
            }
            HWSIM_ATTR_TX_INFO_FLAGS => Self::TXInfoFlags(0),
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
