pub mod class;
mod experience;
mod story_progress;

use self::class::Class;
use story_progress::StoryProgress;
use experience::Experience;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub experience: Experience,
    pub skill_ids: Vec<String>,
    pub story_progress: StoryProgress,
    pub current_area_idx: usize,
    // TODO: add Attributes to determine effectiveness of attacks against the Player
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            class: Class::Basic(class::basic_class::BasicClass::FutureHero),
            experience: Experience {
                level: 1,
                experience_towards_next_level: 0,
            },
            skill_ids: [].to_vec(),
            story_progress: StoryProgress {
                areas_completed: 0,
                current_area_progress: 0,
            },
            current_area_idx: 0,
        }
    }
}
