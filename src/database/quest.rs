use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::Referer;

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

impl Referer for Quest {
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
