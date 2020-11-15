pub trait Referer {
    fn id(&self) -> String;

    fn itemtype_ids(&self) -> Vec<String>;
    fn skill_ids(&self) -> Vec<String>;
    fn task_ids(&self) -> Vec<String>;
    fn quest_ids(&self) -> Vec<String>;
}
