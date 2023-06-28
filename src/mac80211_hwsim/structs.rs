use std::mem::size_of_val;

use netlink_packet_utils::Emitable;

use super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TXRate {
    pub idx: i8,
    pub count: u8,
}

impl Emitable for TXRate {
    fn buffer_len(&self) -> usize {
        size_of_val(&self.idx) + size_of_val(&self.count)
    }

    fn emit(&self, buffer: &mut [u8]) {
        buffer[0] = self.idx as u8;
        buffer[1] = self.count;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TXInfo {
    pub tx_rates_count: i32,
    pub tx_rates: [TXRate; IEEE80211_TX_MAX_RATES],
}

impl Emitable for TXInfo {
    fn buffer_len(&self) -> usize {
        self.tx_rates.iter().map(|r| r.buffer_len()).sum::<usize>()
    }

    fn emit(&self, buffer: &mut [u8]) {
        let mut offset = 0;
        for rate in &self.tx_rates {
            rate.emit(&mut buffer[offset..]);
            offset += rate.buffer_len();
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct ReceiverInfo {
    pub addr: [u8; 6],
    pub signal: u32,
}

impl Emitable for ReceiverInfo {
    fn buffer_len(&self) -> usize {
        6 + 4
    }

    fn emit(&self, buffer: &mut [u8]) {
        buffer[0..6].copy_from_slice(&self.addr);
        buffer[6..10].copy_from_slice(&self.signal.to_le_bytes());
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
#[repr(C, packed)]
pub struct IEEE80211Header {
    pub frame_control: [u8; 2],
    pub duration_id: [u8; 2],
    pub addr1: [u8; 6],
    pub addr2: [u8; 6],
    pub addr3: [u8; 6],
    pub seq_ctrl: [u8; 2],
    pub addr4: [u8; 6],
    pub qos: [u8; 2],
}
