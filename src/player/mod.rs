mod experience;
mod story_progress;
use crate::class::Class;
use story_progress::StoryProgress;
use experience::Experience;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class_id: String,
    pub experience: Experience,
    pub skill_ids: Vec<String>,
    pub story_progress: StoryProgress,
    // TODO: add Attributes to determine effectiveness of attacks against the Player
}

impl Player {
    pub fn new(name: String, class: Class) -> Player {
        Player {
            name,
            class_id: String::from(class.unique_id),
            experience: Experience {
                level: 1,
                experience_towards_next_level: 0,
            },
            skill_ids: [].to_vec(),
            story_progress: StoryProgress {
                areas_completed: 0,
                enemies_defeated_in_current_area: 0,
            },
        }
    }
}
