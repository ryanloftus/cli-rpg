use crate::unit::enemy::{Enemy, EnemyDifficulty};
use serde::{Deserialize, Serialize};

const XP_PER_LEVEL: u16 = 100;
const BASE_XP_FOR_ENEMY_DEFEATED: u16 = 5;
const XP_FOR_AREA_CLEARED: u16 = 100;

const WEAK_ENEMY_XP_MULTIPLIER: u16 = 1;
const STRONG_ENEMY_XP_MULTIPLIER: u16 = 2;
const BOSS_ENEMY_XP_MULTIPLIER: u16 = 20;
const SPECIAL_ENEMY_XP_MULTIPLIER: u16 = 50;

const XP_FOR_SKILL_USED: u16 = 5;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub level: u16,
    pub experience_towards_next_level: u16,
}

impl Experience {
    pub fn new() -> Experience {
        return Experience {
            level: 1,
            experience_towards_next_level: 0,
        };
    }

    fn add_experience(&mut self, experience: u16) -> u16 {
        self.experience_towards_next_level = self.experience_towards_next_level + experience;
        let levels_gained = self.experience_towards_next_level / XP_PER_LEVEL;
        self.level += levels_gained;
        self.experience_towards_next_level %= XP_PER_LEVEL;
        return levels_gained;
    }

    fn get_difficulty_multiplier(enemy: &Enemy) -> u16 {
        match enemy.difficulty {
            EnemyDifficulty::Weak => WEAK_ENEMY_XP_MULTIPLIER,
            EnemyDifficulty::Strong => STRONG_ENEMY_XP_MULTIPLIER,
            EnemyDifficulty::Boss => BOSS_ENEMY_XP_MULTIPLIER,
            EnemyDifficulty::Special => SPECIAL_ENEMY_XP_MULTIPLIER,
        }
    }

    pub fn enemies_defeated(&mut self, enemies: &Vec<Enemy>) -> u16 {
        let mut levels_gained = 0;
        for enemy in enemies {
            levels_gained += self.add_experience(
                BASE_XP_FOR_ENEMY_DEFEATED * Self::get_difficulty_multiplier(enemy),
            );
        }
        return levels_gained;
    }

    pub fn area_cleared(&mut self) -> u16 {
        return self.add_experience(XP_FOR_AREA_CLEARED);
    }

    pub fn skill_used(&mut self) -> u16 {
        return self.add_experience(XP_FOR_SKILL_USED);
    }

    pub fn skill_evolved(&mut self) -> u16 {
        return self.add_experience(XP_PER_LEVEL);
    }
}
