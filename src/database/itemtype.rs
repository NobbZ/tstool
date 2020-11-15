use std::{collections::HashMap, sync::Mutex};

use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref ITEMTYPES: Mutex<HashMap<String, ()>> =
        Mutex::new(HashMap::new());
}
