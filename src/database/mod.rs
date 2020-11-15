use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use slog::{info, Logger};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs,
    path::PathBuf,
    sync::Mutex,
};

lazy_static! {
    static ref TOOLS: Mutex<HashMap<String, Tool>> = Mutex::new(HashMap::new());
    static ref ITEMTYPES: Mutex<HashMap<String, ()>> =
        Mutex::new(HashMap::new());
}

trait Identifyable {
    fn id(&self) -> String;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct SkillBonus {
    pub value: i8,
    pub skill: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct TaskRef {
    pub task: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct QuestRef {
    pub quest: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(deny_unknown_fields)]
pub struct Tool {
    pub name: String,
    pub id: String,
    pub itemtype: String,
    pub skills: Vec<SkillBonus>,
    pub tasks: Option<Vec<TaskRef>>,
    pub quests: Vec<QuestRef>,
}

impl Identifyable for Tool {
    fn id(&self) -> String {
        self.id.clone()
    }
}

impl Display for Tool {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn read_files<'de, T>(log: &Logger, p: &str) -> Vec<T>
where
    T: DeserializeOwned + Display,
{
    let mut result: Vec<T> = Vec::new();

    let folder = [".", "data", p].iter().collect::<PathBuf>();

    info!(log, "entering folder {}", folder.display());

    let files = fs::read_dir(&folder)
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect::<Vec<PathBuf>>();

    for file in files {
        info!(log, "file {}", file.display());
        let f = fs::File::open(file).unwrap();
        let t: T = serde_yaml::from_reader(f).unwrap();
        result.push(t)
    }

    result
}

pub fn load_from_files(log: &Logger) -> Result<(), Vec<String>> {
    let mut errors: HashSet<String> = HashSet::new();

    let tools = read_files::<Tool>(log, "item");

    let mut tool_map = TOOLS.lock().unwrap();
    for tool in &tools {
        tool_map.insert(tool.id.clone(), tool.clone());
    }

    let itemtype_ids: &Vec<_> =
        &tools.iter().map(|t| t.itemtype.clone()).collect();
    info!(log, "tools: {:?}", tools);
    info!(log, "itemtypes; {:?}", itemtype_ids);
    for itemtype_id in itemtype_ids {
        if !ITEMTYPES.lock().unwrap().contains_key(itemtype_id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/itemtype/{}",
                itemtype_id
            ));
        }
    }

    info!(log, "errors: {:?}", errors);

    if !errors.is_empty() {
        return Err(errors.iter().cloned().collect());
    }

    Ok(())
}
