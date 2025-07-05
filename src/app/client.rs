use std::fmt;

use crate::app::Address;

pub type ClientName = String;

#[derive(Debug,Clone,Default)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Client {
    pub name: ClientName,
    pub address: Address,
    pub siret: Option<String>,
    pub code_ape: Option<String>,
    pub tva: Option<String>,
    pub tva_icc: Option<String>,
    pub custom_field: Option<String>,
}
impl Client {
    pub(crate) fn valid(&self) -> bool {
        if self.name.is_empty() { return false; }
        if !self.address.valid() { return false; }
        true
    }
}

impl fmt::Display for Client {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Customize so only `x` and `y` are denoted.
        write!(f, "{}", self.name)
    }
}
