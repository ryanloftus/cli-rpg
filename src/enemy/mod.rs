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
    // TODO: new fn for each type of enemy for simpler enemy generation.
    // should enemies be generated at compile time? or should the be generated as needed?
    // the player should be able to train against an infinite pool of practice enemies
}
