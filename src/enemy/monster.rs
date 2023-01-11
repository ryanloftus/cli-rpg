use super::Enemy;
use rand::{self, Rng};

pub enum MonsterType {
    Slime,
    Goblin,
    Dragon,
    Ogre,
}

pub fn new(monster_type: MonsterType, level: u8) -> Enemy {
    match monster_type {
        MonsterType::Slime => new_slime(level),
        MonsterType::Goblin => new_goblin(level),
        MonsterType::Dragon => new_dragon(level),
        MonsterType::Ogre => new_ogre(level),
    }
}

fn new_slime(level: u8) -> Enemy {
    Enemy {
        name: String::from("Slime"),
        level,
        skills: Vec::new(),
    }
}

fn new_goblin(level: u8) -> Enemy {
    Enemy {
        name: String::from("Goblin"),
        level,
        skills: Vec::new(),
    }
}

fn new_dragon(level: u8) -> Enemy {
    Enemy {
        name: String::from("Dragon"),
        level,
        skills: Vec::new(),
    }
}

const SHREK_CHANCE: u8 = 1;

fn new_ogre(level: u8) -> Enemy {
    if rand::thread_rng().gen_range(0..=100) < SHREK_CHANCE {
        Enemy {
            name: String::from("Shrek"),
            level: 100,
            skills: Vec::new(),
        }
    } else {
        Enemy {
            name: String::from("Ogre"),
            level,
            skills: Vec::new(),
        }
    }
}
