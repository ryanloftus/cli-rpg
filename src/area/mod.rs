mod plains;
mod forest;
mod mountains;
mod islands;
mod story;

use crate::player::Player;
use plains::PLAINS;
use forest::FOREST;
use mountains::MOUNTAINS;
use islands::ISLANDS;
use story::StoryComponent;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
}

impl Area {
    pub fn start(mut player: &Player) {
        // TODO
    }
}

pub const AREAS: &[Area] = &[PLAINS, FOREST, ISLANDS, MOUNTAINS];
