mod experience;
use crate::class::Class;
use crate::skill::Skill;
use experience::Experience;

pub struct Player {
    pub name: String,
    pub class: Class,
    pub experience: Experience,
    pub skills: Vec<Skill>,
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
        }
    }
}
