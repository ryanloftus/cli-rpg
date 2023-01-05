use crate::enemy::Enemy;
use super::Area;

const ENEMIES: &[Enemy] = &[]; // TODO: add enemies

pub const FOREST: Area = Area {
    name: "The Forest",
    unique_id: "F",
    enemies: ENEMIES,
};
