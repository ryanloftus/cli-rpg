pub mod class;
mod experience;
mod story_progress;

use crate::{
    area::Area,
    prompt::PromptOption,
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
    pub current_area: Area, // TODO: actually use this in game/mod.rs
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
            current_area: Area::Kingdom,
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
    Max MP: {}
    Strength: {}
    Magic: {}
    Defense: {}
    Magic Resist: {}
    Speed: {}
    Skill: {}
    Luck: {}
",
            self.stats.max_health,
            self.stats.max_mp,
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
