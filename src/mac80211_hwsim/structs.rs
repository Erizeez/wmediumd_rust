use std::mem::size_of_val;

use netlink_packet_utils::Emitable;

use super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct TXInfo {
    pub idx: i8,
    pub count: u8,
}

impl Emitable for TXInfo {
    fn buffer_len(&self) -> usize {
        size_of_val(&self.idx) + size_of_val(&self.count)
    }

    fn emit(&self, buffer: &mut [u8]) {
        buffer[0] = self.idx as u8;
        buffer[1] = self.count;
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default, Copy)]
pub struct TXInfoFlag {
    pub idx: i8,
    pub flags: u16,
}

impl Emitable for TXInfoFlag {
    fn buffer_len(&self) -> usize {
        size_of_val(&self.idx) + size_of_val(&self.flags)
    }

    fn emit(&self, buffer: &mut [u8]) {
        buffer[0] = self.idx as u8;
        buffer[1..].copy_from_slice(&self.flags.to_le_bytes());
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
