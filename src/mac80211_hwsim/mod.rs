use genetlink::GenetlinkHandle;

use crate::{config::Radio, structs::GenlNewRadio};

use self::{constants::ETH_ALEN, ctrl::GenlAutoConstruct};

pub mod constants;
pub mod ctrl;
pub mod structs;

pub type MACAddress = [u8; ETH_ALEN];

pub async fn new_radio_nl(handle: &mut GenetlinkHandle, radio: GenlNewRadio) {
    handle
        .notify(radio.generate_genl_message())
        .await
        .expect("send new radio error");
}
