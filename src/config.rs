use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

use crate::mac80211_hwsim::constants::ETH_ALEN;
use crate::structs::GenlNewRadio;
use crate::{HwsimRadio, HwsimRadios};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Radio {
    pub channels: u32,
    pub support_p2p_device: bool,
    pub use_chanctx: bool,
    pub destroy_radio_on_close: bool,
    // radio_name: String,
    pub no_vif: bool,
    pub perm_addr: [u8; ETH_ALEN],
}

impl TryInto<GenlNewRadio> for Radio {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<GenlNewRadio, Self::Error> {
        Ok(GenlNewRadio {
            channels: self.channels,
            reg_hint_alpha2: "".to_owned(),
            reg_custom_reg: 0,
            reg_strict_reg: false,
            support_p2p_device: self.support_p2p_device,
            use_chanctx: self.use_chanctx,
            destroy_radio_on_close: self.destroy_radio_on_close,
            radio_name: "".to_owned(),
            no_vif: self.no_vif,
            perm_addr: self.perm_addr,
            iftype_support: 0,
            cipher_support: vec![],
        })
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct Config {
    pub radios: Vec<Radio>,
}

impl TryInto<HwsimRadios> for Config {
    type Error = anyhow::Error;

    fn try_into(self) -> Result<HwsimRadios, Self::Error> {
        let mut hwsim_radios = HwsimRadios::default();
        for radio in &self.radios {
            hwsim_radios.radios.push(HwsimRadio {
                addr: radio.perm_addr,
                hw_addr: radio.perm_addr,
            })
        }
        Ok(hwsim_radios)
    }
}

pub fn load_config(config_path: &str) -> Config {
    let file = File::open(config_path).expect("fail to open config");

    // let deserialized_config: Config =
    serde_yaml::from_reader(file).expect("deserialize error")
}

pub fn load_radio(config_path: &str) -> Radio {
    let file = File::open(config_path).expect("fail to open config");

    // let deserialized_config: Config =
    serde_yaml::from_reader(file).expect("deserialize error")
}
