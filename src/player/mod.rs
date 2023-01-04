use crate::class::Class;
use crate::skill::Skill;

pub struct Player {
    pub name: String,
    pub class: Class,
    pub level: u32,
    pub skills: Vec<Skill>,
}
