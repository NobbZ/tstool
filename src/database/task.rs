use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{Referer, RegionRef, SkillBonus};

lazy_static! {
    pub(crate) static ref TASKS: Mutex<HashMap<String, Task>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TaskRef {
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub name: String,
    pub id: String,
    pub duration: String, // TODO: make this some proper duration type
    pub cost: i64,
    pub difficulty: Vec<SkillBonus>,
    pub result: Vec<HashMap<String, String>>, // TODO: Proper enum with variants
    pub regions: Vec<RegionRef>,
}

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Referer for Task {
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
