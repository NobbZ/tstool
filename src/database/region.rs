use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref REGIONS: Mutex<HashMap<String, Region>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct RegionRef {
    pub region: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Region {
    pub id: String,
}

impl Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Quest::fmt not implemented yet")
    }
}
