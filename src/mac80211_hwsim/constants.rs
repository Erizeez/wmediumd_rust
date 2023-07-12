pub const FAMILY_NAME: &str = "MAC80211_HWSIM";
pub const ETH_ALEN: usize = 6;

pub const HWSIM_ATTR_UNSPEC: u16 = 0;
pub const HWSIM_ATTR_ADDR_RECEIVER: u16 = 1;
pub const HWSIM_ATTR_ADDR_TRANSMITTER: u16 = 2;
pub const HWSIM_ATTR_FRAME: u16 = 3;
pub const HWSIM_ATTR_FLAGS: u16 = 4;
pub const HWSIM_ATTR_RX_RATE: u16 = 5;
pub const HWSIM_ATTR_SIGNAL: u16 = 6;
pub const HWSIM_ATTR_TX_INFO: u16 = 7;
pub const HWSIM_ATTR_COOKIE: u16 = 8;
pub const HWSIM_ATTR_CHANNELS: u16 = 9;
pub const HWSIM_ATTR_RADIO_ID: u16 = 10;
pub const HWSIM_ATTR_REG_HINT_ALPHA2: u16 = 11;
pub const HWSIM_ATTR_REG_CUSTOM_REG: u16 = 12;
pub const HWSIM_ATTR_REG_STRICT_REG: u16 = 13;
pub const HWSIM_ATTR_SUPPORT_P2P_DEVICE: u16 = 14;
pub const HWSIM_ATTR_USE_CHANCTX: u16 = 15;
pub const HWSIM_ATTR_DESTROY_RADIO_ON_CLOSE: u16 = 16;
pub const HWSIM_ATTR_RADIO_NAME: u16 = 17;
pub const HWSIM_ATTR_NO_VIF: u16 = 18;
pub const HWSIM_ATTR_FREQ: u16 = 19;
pub const HWSIM_ATTR_PAD: u16 = 20;
pub const HWSIM_ATTR_TX_INFO_FLAGS: u16 = 21;
pub const HWSIM_ATTR_PERM_ADDR: u16 = 22;
pub const HWSIM_ATTR_IFTYPE_SUPPORT: u16 = 23;
pub const HWSIM_ATTR_CIPHER_SUPPORT: u16 = 24;
pub const HWSIM_ATTR_SM_POINTER: u16 = 25;
pub const HWSIM_ATTR_SM_PAGE_NUM: u16 = 26;

pub const HWSIM_CMD_REGISTER: u8 = 1;
pub const HWSIM_CMD_FRAME: u8 = 2;
pub const HWSIM_CMD_TX_INFO_FRAME: u8 = 3;
pub const HWSIM_CMD_NEW_RADIO: u8 = 4;
pub const HWSIM_CMD_DEL_RADIO: u8 = 5;
pub const HWSIM_CMD_GET_RADIO: u8 = 6;
pub const HWSIM_CMD_ADD_MAC_ADDR: u8 = 7;
pub const HWSIM_CMD_DEL_MAC_ADDR: u8 = 8;

// pub const IEEE80211_AVAILABLE_RATES: isize = 12;
pub const IEEE80211_TX_MAX_RATES: usize = 4;
// pub const IEEE80211_NUM_ACS: isize = 4;

pub const MICROSECONDS_TO_NANOSECONDS: i64 = 1000;
