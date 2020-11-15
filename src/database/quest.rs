use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub(crate) static ref QUESTS: Mutex<HashMap<String, Quest>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct QuestRef {
    pub quest: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Quest {
    pub id: String,
}

impl Display for Quest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Quest::fmt not implemented yet")
    }
}
