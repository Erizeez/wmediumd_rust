pub mod nlas;

use anyhow::Context;
use netlink_packet_core::{NetlinkHeader, NetlinkMessage, NLM_F_REQUEST};
use netlink_packet_generic::{GenlFamily, GenlHeader, GenlMessage};
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
    NewRadio,
    DelRadio,
    GetRadio,
    AddMACAddr,
    DelMACAddr,
}

impl From<HwsimCmd> for u8 {
    fn from(value: HwsimCmd) -> Self {
        use HwsimCmd::*;
        match value {
            Register => HWSIM_CMD_REGISTER,
            Frame => HWSIM_CMD_FRAME,
            TXInfoFrame => HWSIM_CMD_TX_INFO_FRAME,
            NewRadio => HWSIM_CMD_NEW_RADIO,
            DelRadio => HWSIM_CMD_DEL_RADIO,
            GetRadio => HWSIM_CMD_GET_RADIO,
            AddMACAddr => HWSIM_CMD_ADD_MAC_ADDR,
            DelMACAddr => HWSIM_CMD_DEL_MAC_ADDR,
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
            HWSIM_CMD_NEW_RADIO => NewRadio,
            HWSIM_CMD_DEL_RADIO => DelRadio,
            HWSIM_CMD_GET_RADIO => GetRadio,
            HWSIM_CMD_ADD_MAC_ADDR => AddMACAddr,
            HWSIM_CMD_DEL_MAC_ADDR => DelMACAddr,
            cmd => return Err(DecodeError::from(format!("Unknown control command: {cmd}"))),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GenlMAC {
    pub cmd: HwsimCmd,
    pub nlas: Vec<HwsimAttrs>,
}

pub trait GenlAutoConstruct {
    fn generate_genl_message(&self) -> NetlinkMessage<GenlMessage<GenlMAC>>;
    fn parse(data: GenlMAC) -> Self;
}

pub fn parse_genl_message<T>(data: GenlMAC) -> T
where
    T: GenlAutoConstruct,
{
    T::parse(data)
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
