pub mod class;
mod story_progress;

use crate::{
    prompt::PromptOption,
    unit::skill::Skill,
    unit::stats::{Stats, BASE_HEALTH},
};

use self::class::Class;
use serde::{Deserialize, Serialize};
use story_progress::StoryProgress;

use super::experience::Experience;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub class: Class,
    pub experience: Experience,
    pub skills: Vec<Skill>,
    pub stats: Stats,
    pub story_progress: StoryProgress,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name,
            class: Class::FutureHero,
            experience: Experience::new(),
            skills: Vec::new(),
            stats: Stats {
                max_health: BASE_HEALTH,
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
        }
    }

    pub fn print_summary(&self) {
        println!("Name: {}", self.name);
        println!("Class: {}", self.class.option_name());
        println!("Level: {}", self.experience.level);
        println!(
            "Experience towards next level: {} / 100",
            self.experience.experience_towards_next_level
        );
        print!(
            "Stats:
    Max Health: {}
    Strength: {}
    Magic: {}
    Defense: {}
    Magic Resist: {}
    Speed: {}
    Skill: {}
    Luck: {}
",
            self.stats.max_health,
            self.stats.strength,
            self.stats.magic,
            self.stats.defense,
            self.stats.magic_resist,
            self.stats.speed,
            self.stats.skill,
            self.stats.luck
        );
    }
}
