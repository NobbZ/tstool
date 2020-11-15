use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Skill {
    pub id: String,
}

impl Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!("Quest::fmt not implemented yet")
    }
}
