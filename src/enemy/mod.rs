use std::borrow::Cow;
use crate::skill::Skill;

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: Cow<'static, str>,
    pub level: u8,
    pub skills: Vec<Skill>,
    // TODO: add stats
    // TODO: add Attributes to determine effectiveness of attacks against this foe
}

impl Enemy {
    pub const fn new_const(name: &'static str, level: u8, skills: &'static [Skill]) -> Enemy {
        Enemy {
            name: Cow::Borrowed(name),
            level: level,
            skills: skills.to_vec(),
        }
    }

    // TODO: new fn for each type of enemy for simpler enemy generation.
}
