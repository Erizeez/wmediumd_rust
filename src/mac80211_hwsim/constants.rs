pub const FAMILY_NAME: &str = "MAC80211_HWSIM";
pub const ETH_ALEN: u16 = 6;

pub const HWSIM_ATTR_ADDR_TRANSMITTER: u16 = 2;
pub const HWSIM_ATTR_FLAGS: u16 = 4;
pub const HWSIM_ATTR_RX_RATE: u16 = 5;
pub const HWSIM_ATTR_SIGNAL: u16 = 6;
pub const HWSIM_ATTR_TX_INFO: u16 = 7;
pub const HWSIM_ATTR_COOKIE: u16 = 8;
pub const HWSIM_ATTR_FREQ: u16 = 19;
pub const HWSIM_ATTR_TX_INFO_FLAGS: u16 = 21;
pub const HWSIM_ATTR_FRAME_HEADER: u16 = 25;
pub const HWSIM_ATTR_FRAME_LENGTH: u16 = 26;
pub const HWSIM_ATTR_RECEIVER_INFO: u16 = 28;
pub const HWSIM_ATTR_FRAME_TIMESTAMP: u16 = 29;

pub const HWSIM_CMD_REGISTER: u8 = 1;
pub const HWSIM_CMD_FRAME: u8 = 2;
pub const HWSIM_CMD_TX_INFO_FRAME: u8 = 3;
pub const HWSIM_YAWMD_TX_INFO: u8 = 9;
pub const HWSIM_YAWMD_RX_INFO: u8 = 10;

pub const IEEE80211_AVAILABLE_RATES: isize = 12;
pub const IEEE80211_TX_MAX_RATES: usize = 4;
pub const IEEE80211_NUM_ACS: isize = 4;

pub const MICROSECONDS_TO_NANOSECONDS: i64 = 1000;
