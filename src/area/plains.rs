use crate::enemy::Enemy;
use super::Area;

const ENEMIES: &[Enemy] = &[]; // TODO: add enemies

pub const PLAINS: Area = Area {
    name: "The Plains",
    unique_id: "P",
    enemies: ENEMIES,
};
