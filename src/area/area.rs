use std::borrow::Cow;
use crate::enemy::Enemy;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: Cow<'static, str>,
    pub unique_id: Cow<'static, str>,
    pub enemies: Vec<Enemy>,
    // TODO: add progress
}

impl Area {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }

    const fn new_const(name: &'static str, unique_id: &'static str, enemies: &'static [Enemy]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            unique_id: Cow::Borrowed(unique_id),
            enemies: enemies.to_vec(),
        }
    }
}

mod areas {
    use super::Area;

    pub const PLAINS: Area = Area::new_const("The Plains", "P"); // TODO
    pub const FOREST: Area = Area::new_const("The Forest", "F");
    pub const MOUNTAINS: Area = Area::new_const("The Mountains", "M");
    pub const ISLANDS: Area = Area::new_const("The Islands", "I");
}
