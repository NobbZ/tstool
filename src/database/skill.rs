use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::Referer;

lazy_static! {
    pub(crate) static ref SKILLS: Mutex<HashMap<String, Skill>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SkillBonus {
    pub value: i8,
    pub skill: String,
}

// TODO: Better name and place
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SkillTraining {
    pub limit: i8,
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub id: String,
    pub name: String,
    pub training: Vec<SkillTraining>,
}

impl Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Referer for Skill {
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
