use lazy_static::lazy_static;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use slog::{info, Logger};
use std::{
    collections::{HashMap, HashSet},
    fmt::{self, Display},
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

lazy_static! {
    static ref TOOLS: Mutex<HashMap<String, Tool>> = Mutex::new(HashMap::new());
    static ref ITEMTYPES: Mutex<HashMap<String, ()>> =
        Mutex::new(HashMap::new());
    static ref SKILLS: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
    static ref TASKS: Mutex<HashMap<String, Task>> = Mutex::new(HashMap::new());
    static ref QUESTS: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
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
pub struct RegionRef {
    pub region: String,
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
    pub tasks: Vec<TaskRef>,
    pub quests: Vec<QuestRef>,
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

impl Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

fn read_files<'de, T, P>(log: &Logger, p: P) -> Vec<T>
where
    T: DeserializeOwned + Display,
    P: AsRef<Path>,
{
    let mut result: Vec<T> = Vec::new();

    let folder: &Path = p.as_ref();

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

fn get_path<B, F>(base: B, folder: F) -> PathBuf
where
    B: AsRef<Path>,
    F: AsRef<Path>,
{
    let mut p = PathBuf::new();

    p.push(base);
    p.push("data");
    p.push(folder);

    p
}

pub fn load_from_files<P>(log: &Logger, base: P) -> Result<(), Vec<String>>
where
    P: AsRef<Path>,
{
    let mut errors: HashSet<String> = HashSet::new();

    let tools = read_files::<Tool, _>(log, &get_path(&base, "item"));
    let tasks = read_files::<Task, _>(log, &get_path(&base, "task"));

    let mut tool_map = TOOLS.lock().unwrap();
    for tool in &tools {
        tool_map.insert(tool.id.clone(), tool.clone());
    }

    for task in &tasks {
        TASKS.lock().unwrap().insert(task.id.clone(), task.clone());
    }

    let itemtype_ids: &Vec<_> =
        &tools.iter().map(|t| t.itemtype.clone()).collect();
    let skill_ids: &Vec<_> = &tools
        .iter()
        .flat_map(|t| t.skills.iter().map(|s| s.skill.clone()))
        .collect();
    let task_ids: &Vec<_> = &tools
        .iter()
        .flat_map(|tool| tool.tasks.iter().map(|task| task.task.clone()))
        .collect();
    let quest_ids: &Vec<_> = &tools
        .iter()
        .flat_map(|tool| tool.quests.iter().map(|quest| quest.quest.clone()))
        .collect();
    info!(log, "tools: {:?}", tools);
    info!(log, "itemtypes: {:?}", itemtype_ids);
    info!(log, "skills: {:?}", skill_ids);
    info!(log, "tasks: {:?}", task_ids);
    info!(log, "quests: {:?}", quest_ids);
    for itemtype_id in itemtype_ids {
        if !ITEMTYPES.lock().unwrap().contains_key(itemtype_id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/itemtype/{}",
                itemtype_id
            ));
        }
    }
    for skill_id in skill_ids {
        if !SKILLS.lock().unwrap().contains_key(skill_id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/skill/{}",
                skill_id,
            ));
        }
    }
    for task_id in task_ids {
        if !SKILLS.lock().unwrap().contains_key(task_id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/task/{}",
                task_id,
            ));
        }
    }
    for quest_id in quest_ids {
        if !SKILLS.lock().unwrap().contains_key(quest_id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/quest/{}",
                quest_id,
            ));
        }
    }

    info!(log, "errors: {:?}", errors);

    if !errors.is_empty() {
        return Err(errors.iter().cloned().collect());
    }

    Ok(())
}
