use std::mem::size_of_val;

use netlink_packet_utils::Emitable;

use super::constants::*;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct Frame {
    pub header: IEEE80211Header,
    pub payload: Vec<u8>,
}

impl Emitable for Frame {
    fn buffer_len(&self) -> usize {
        self.header.buffer_len() + self.payload.len()
    }

    fn emit(&self, buffer: &mut [u8]) {
        self.header.emit(buffer);
        buffer[self.header.buffer_len()..].copy_from_slice(&self.payload);
    }
}

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

impl Emitable for IEEE80211Header {
    fn buffer_len(&self) -> usize {
        2 + 2 + 6 + 6 + 6 + 2 + 6 + 2
    }

    fn emit(&self, buffer: &mut [u8]) {
        buffer[0..2].copy_from_slice(&self.frame_control);
        buffer[2..4].copy_from_slice(&self.duration_id);
        buffer[4..10].copy_from_slice(&self.addr1);
        buffer[10..16].copy_from_slice(&self.addr2);
        buffer[16..22].copy_from_slice(&self.addr3);
        buffer[22..24].copy_from_slice(&self.seq_ctrl);
        buffer[24..30].copy_from_slice(&self.addr4);
        buffer[30..32].copy_from_slice(&self.qos);
    }
}
