pub mod class;
mod experience;
mod story_progress;

use crate::{
    skill::Skill,
    stats::{Stats, BASE_HEALTH, BASE_MP},
};

use self::class::Class;
use experience::Experience;
use serde::{Deserialize, Serialize};
use story_progress::StoryProgress;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub experience: Experience,
    pub skills: Vec<Skill>,
    pub stats: Stats,
    pub story_progress: StoryProgress,
    pub current_area_idx: usize,
    // TODO: add Attributes to determine effectiveness of attacks against the Player
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            class: Class::FutureHero,
            experience: Experience {
                level: 1,
                experience_towards_next_level: 0,
            },
            skills: [].to_vec(),
            stats: Stats {
                max_health: BASE_HEALTH,
                max_mp: BASE_MP,
                strength: 1,
                magic: 1,
                defense: 1,
                magic_resist: 1,
                speed: 1,
                skill: 1,
                luck: 1,
            },
            story_progress: StoryProgress {
                areas_completed: 0,
                current_area_progress: 0,
            },
            current_area_idx: 0,
        }
    }
}
