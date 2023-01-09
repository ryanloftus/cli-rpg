pub mod monster;
pub mod soldier;

use crate::skill::Skill;
use rand::{self, Rng};

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub level: u8,
    pub skills: Vec<Skill>,
    // TODO: add stats
    // TODO: add Attributes to determine effectiveness of attacks against this foe
}

pub enum EnemyType {
    Monster(monster::MonsterType),
    Soldier { faction: String, soldier_type: soldier::SoldierType },
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, min_level: u8, max_level: u8) -> Enemy {
        let level = rand::thread_rng().gen_range(min_level..=max_level);
        match enemy_type {
            EnemyType::Monster(monster_type) => monster::new(monster_type, level),
            EnemyType::Soldier { faction, soldier_type } => soldier::new(soldier_type, faction, level),
        }
    }
}
