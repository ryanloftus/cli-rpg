pub mod monster;
pub mod soldier;

use crate::{
    stats::{StatMultiplier, Stats},
    unit::skill::Skill,
};
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
    pub level: u16,
    pub skills: Vec<Skill>,
    pub difficulty: EnemyDifficulty,
    pub stats: Stats,
    // TODO: add Attributes to determine effectiveness of attacks against this foe
    // TODO: actually give enemies unique skills and attributes
}

pub enum EnemyType {
    Monster {
        name_prefix: Option<String>,
        monster_type: monster::MonsterType,
    },
    Soldier {
        faction: String,
        soldier_type: soldier::SoldierType,
    },
}

impl Enemy {
    pub fn new(enemy_type: EnemyType, base_level: u16, area_progress: u8) -> Enemy {
        let level = calculate_level(base_level, area_progress);
        return match enemy_type {
            EnemyType::Monster {
                name_prefix,
                monster_type,
            } => monster::new(monster_type, name_prefix, level),
            EnemyType::Soldier {
                faction,
                soldier_type,
            } => soldier::new(soldier_type, faction, level),
        };
    }

    pub fn new_boss(
        name: String,
        level: u16,
        skills: Vec<Skill>,
        stat_multipliers: Vec<StatMultiplier>,
    ) -> Enemy {
        return Enemy {
            name,
            level,
            skills,
            difficulty: EnemyDifficulty::Boss,
            stats: Stats::new(level, stat_multipliers),
        };
    }
}

const ENEMY_LEVEL_VARIANCE: f32 = 0.05;

fn calculate_level(base_level: u16, area_progress: u8) -> u16 {
    let avg_level: f32 = f32::from(base_level) + f32::from(area_progress) / 15.0;
    let min_level = (avg_level * (1.0 - ENEMY_LEVEL_VARIANCE)).round() as u16;
    let max_level = (avg_level * (1.0 + ENEMY_LEVEL_VARIANCE)).round() as u16;
    return rand::thread_rng().gen_range(min_level..=max_level);
}
