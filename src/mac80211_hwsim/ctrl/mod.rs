pub mod nlas;

use anyhow::Context;
use netlink_packet_generic::{GenlFamily, GenlHeader};
use netlink_packet_utils::{
    nla::{self, NlaBuffer, NlasIterator},
    DecodeError, Emitable, Parseable, ParseableParametrized,
};

use self::nlas::*;
use super::constants::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HwsimCmd {
    Register,
    Frame,
    TXInfoFrame,
    YawmdTXInfo,
    YawmdRXInfo,
}

impl From<HwsimCmd> for u8 {
    fn from(value: HwsimCmd) -> Self {
        use HwsimCmd::*;
        match value {
            Register => HWSIM_CMD_REGISTER,
            Frame => HWSIM_CMD_FRAME,
            TXInfoFrame => HWSIM_CMD_TX_INFO_FRAME,
            YawmdTXInfo => HWSIM_YAWMD_TX_INFO,
            YawmdRXInfo => HWSIM_YAWMD_RX_INFO,
        }
    }
}

impl TryFrom<u8> for HwsimCmd {
    type Error = DecodeError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use HwsimCmd::*;
        Ok(match value {
            HWSIM_CMD_REGISTER => Register,
            HWSIM_CMD_FRAME => Frame,
            HWSIM_CMD_TX_INFO_FRAME => TXInfoFrame,
            HWSIM_YAWMD_TX_INFO => YawmdTXInfo,
            HWSIM_YAWMD_RX_INFO => YawmdRXInfo,
            cmd => return Err(DecodeError::from(format!("Unknown control command: {cmd}"))),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenlMAC {
    pub cmd: HwsimCmd,
    pub nlas: Vec<HwsimAttrs>,
}

impl GenlFamily for GenlMAC {
    fn family_name() -> &'static str {
        FAMILY_NAME
    }

    fn command(&self) -> u8 {
        self.cmd.into()
    }

    fn version(&self) -> u8 {
        1
    }
}

impl Emitable for GenlMAC {
    fn emit(&self, buffer: &mut [u8]) {
        self.nlas.as_slice().emit(buffer)
    }

    fn buffer_len(&self) -> usize {
        self.nlas.as_slice().buffer_len()
    }
}

impl ParseableParametrized<[u8], GenlHeader> for GenlMAC {
    fn parse_with_param(buf: &[u8], header: GenlHeader) -> Result<Self, DecodeError> {
        Ok(Self {
            cmd: header.cmd.try_into()?,
            nlas: parse_nlas(buf)?,
        })
    }
}

fn parse_nlas(buf: &[u8]) -> Result<Vec<HwsimAttrs>, DecodeError> {
    let nlas = NlasIterator::new(buf)
        .map(|nla| nla.and_then(|nla| HwsimAttrs::parse(&nla)))
        .collect::<Result<Vec<_>, _>>()
        .context("failed to parse mac80211_hwsim attributes")?;

    Ok(nlas)
}
