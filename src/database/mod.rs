use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    fs,
    path::{Path, PathBuf},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::de::DeserializeOwned;
use slog::{debug, info, Logger};

pub use quest::QuestRef;
pub use referer::Referer;
pub use region::RegionRef;
pub use skill::SkillBonus;
pub use task::{Task, TaskRef};
pub use tool::Tool;

mod quest;
mod referer;
mod region;
mod skill;
mod task;
mod tool;

lazy_static! {
    static ref TOOLS: Mutex<HashMap<String, Tool>> = Mutex::new(HashMap::new());
    static ref ITEMTYPES: Mutex<HashMap<String, ()>> =
        Mutex::new(HashMap::new());
    static ref SKILLS: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
    static ref TASKS: Mutex<HashMap<String, Task>> = Mutex::new(HashMap::new());
    static ref QUESTS: Mutex<HashMap<String, ()>> = Mutex::new(HashMap::new());
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
        &tools.iter().flat_map(|tool| tool.itemtype_ids()).collect();
    let skill_ids: &Vec<_> =
        &tools.iter().flat_map(|tool| tool.skill_ids()).collect();
    let task_ids: &Vec<_> =
        &tools.iter().flat_map(|tool| tool.task_ids()).collect();
    let quest_ids: &Vec<_> =
        &tools.iter().flat_map(|tool| tool.quest_ids()).collect();

    debug!(log, "tools: {:?}", tools);
    debug!(log, "itemtypes: {:?}", itemtype_ids);
    debug!(log, "skills: {:?}", skill_ids);
    debug!(log, "tasks: {:?}", task_ids);
    debug!(log, "quests: {:?}", quest_ids);

    check_ids(&mut errors, &ITEMTYPES, itemtype_ids, "itemtype");
    check_ids(&mut errors, &SKILLS, skill_ids, "skill");
    check_ids(&mut errors, &TASKS, task_ids, "task");
    check_ids(&mut errors, &QUESTS, quest_ids, "quest");

    debug!(log, "collected errors: {:?}", errors);

    if !errors.is_empty() {
        return Err(errors.iter().cloned().collect());
    }

    Ok(())
}

fn check_ids<T>(
    errors: &mut HashSet<String>,
    hm: &Mutex<HashMap<String, T>>,
    ids: &[String],
    label: &str,
) {
    for id in ids {
        if !hm.lock().unwrap().contains_key(id) {
            errors.insert(format!(
                "http://guide.theriansaga-wiki.ru/en/{label}/{id}",
                label = label,
                id = id
            ));
        }
    }
}
