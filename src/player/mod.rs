mod experience;
mod story_progress;
use crate::class::Class;
use crate::skill::Skill;
use story_progress::StoryProgress;
use experience::Experience;

pub struct Player {
    pub name: String,
    pub class: Class,
    pub experience: Experience,
    pub skills: Vec<Skill>,
    pub story_progress: StoryProgress,
    // TODO: add Attributes to determine effectiveness of attacks against the Player
}

impl Player {
    pub fn new(name: String, class: Class) -> Player {
        Player {
            name,
            class,
            experience: Experience {
                level: 1,
                experience_towards_next_level: 0,
            },
            skills: [].to_vec(),
            story_progress: StoryProgress {
                areas_completed: 0,
                enemies_defeated_in_current_area: 0,
            },
        }
    }
}
