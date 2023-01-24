pub mod monster;
pub mod soldier;

use crate::skill::Skill;
use rand::{self, Rng};

#[derive(Debug, Clone)]
pub enum EnemyDifficulty {
    Weak,
    Strong,
    Boss,
    Special,
}

#[derive(Debug, Clone)]
pub struct Enemy {
    pub name: String,
    pub level: u8,
    pub skills: Vec<Skill>,
    pub difficulty: EnemyDifficulty,
    // TODO: add stats
    // TODO: add Attributes to determine effectiveness of attacks against this foe
}

pub enum EnemyType {
    Monster(monster::MonsterType),
    Soldier {
        faction: String,
        soldier_type: soldier::SoldierType,
    },
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, base_level: u8, area_progress: u8) -> Enemy {
        let level = calculate_level(base_level, area_progress);
        return match enemy_type {
            EnemyType::Monster(monster_type) => monster::new(monster_type, level),
            EnemyType::Soldier {
                faction,
                soldier_type,
            } => soldier::new(soldier_type, faction, level),
        };
    }
}

const ENEMY_LEVEL_VARIANCE: f32 = 0.05;

fn calculate_level(base_level: u8, area_progress: u8) -> u8 {
    let avg_level: f32 = f32::from(base_level) + f32::from(area_progress) / 15.0;
    let min_level = (avg_level * (1.0 - ENEMY_LEVEL_VARIANCE)).round() as u8;
    let max_level = (avg_level * (1.0 + ENEMY_LEVEL_VARIANCE)).round() as u8;
    return rand::thread_rng().gen_range(min_level..=max_level);
}
