use std::{
    collections::HashMap,
    fmt::{self, Display},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use super::{skill::SkillBonus, QuestRef, Referer, TaskRef};

lazy_static! {
    pub(super) static ref TOOLS: Mutex<HashMap<String, Tool>> =
        Mutex::new(HashMap::new());
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub name: String,
    pub id: String,
    pub itemtype: String,
    pub skills: Vec<SkillBonus>,
    pub tasks: Vec<TaskRef>,
    pub quests: Vec<QuestRef>,
}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Referer for Tool {
    fn id(&self) -> String {
        self.id.clone()
    }

    fn itemtype_ids(&self) -> Vec<String> {
        vec![self.itemtype.clone()]
    }

    fn skill_ids(&self) -> Vec<String> {
        self.skills
            .iter()
            .map(|skill| skill.skill.clone())
            .collect()
    }

    fn task_ids(&self) -> Vec<String> {
        self.tasks.iter().map(|task| task.task.clone()).collect()
    }

    fn quest_ids(&self) -> Vec<String> {
        self.quests
            .iter()
            .map(|quest| quest.quest.clone())
            .collect()
    }
}
