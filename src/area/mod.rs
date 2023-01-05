mod plains;
mod forest;
mod mountains;
mod islands;
mod story;

use crate::enemy::Enemy;
use plains::PLAINS;
use forest::FOREST;
use mountains::MOUNTAINS;
use islands::ISLANDS;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub enemies: &'static [Enemy],
    // pub story: &'static [AreaStory],
}

impl Area {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }
}

const AREAS: &[Area] = &[PLAINS, FOREST, ISLANDS, MOUNTAINS];
