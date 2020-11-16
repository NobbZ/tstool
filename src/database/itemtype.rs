use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::Referer;

lazy_static! {
    pub(crate) static ref ITEMTYPES: Mutex<HashMap<String, Itemtype>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Itemtype {
    pub id: String,
    pub name: String,
}

impl Display for Itemtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Referer for Itemtype {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn itemtype_ids(&self) -> Vec<String> {
        todo!()
    }

    fn skill_ids(&self) -> Vec<String> {
        todo!()
    }

    fn task_ids(&self) -> Vec<String> {
        todo!()
    }

    fn quest_ids(&self) -> Vec<String> {
        todo!()
    }
}
