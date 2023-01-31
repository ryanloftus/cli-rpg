use crate::stats::Stats;

use super::Enemy;
use rand::{self, Rng};

pub enum MonsterType {
    Slime,
    Goblin,
    Dragon,
    Ogre,
    Demon,
    DemonicBeast,
}

pub fn new(monster_type: MonsterType, name_prefix: Option<String>, level: u16) -> Enemy {
    let mut monster = match monster_type {
        MonsterType::Slime => new_slime(level),
        MonsterType::Goblin => new_goblin(level),
        MonsterType::Dragon => new_dragon(level),
        MonsterType::Ogre => new_ogre(level),
        MonsterType::Demon => new_demon(level),
        MonsterType::DemonicBeast => new_demonic_beast(level),
    };
    if let Some(name_prefix) = name_prefix {
        monster.name = name_prefix + &monster.name;
    }
    return monster;
}

fn new_slime(level: u16) -> Enemy {
    Enemy {
        name: String::from("Slime"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(level, Vec::new()),
    }
}

fn new_goblin(level: u16) -> Enemy {
    Enemy {
        name: String::from("Goblin"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(level, Vec::new()),
    }
}

fn new_dragon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Dragon"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats::new(level, Vec::new()),
    }
}

const SHREK_CHANCE: u16 = 5;

fn new_ogre(level: u16) -> Enemy {
    if rand::thread_rng().gen_range(0..=100) < SHREK_CHANCE {
        Enemy {
            name: String::from("Shrek"),
            level: 100,
            skills: Vec::new(),
            difficulty: super::EnemyDifficulty::Special,
            stats: Stats::new(500, Vec::new()),
        }
    } else {
        Enemy {
            name: String::from("Ogre"),
            level,
            skills: Vec::new(),
            difficulty: super::EnemyDifficulty::Strong,
            stats: Stats::new(level, Vec::new()),
        }
    }
}

fn new_demon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demon"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(level, Vec::new()),
    }
}

fn new_demonic_beast(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demonic Beast"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats::new(level, Vec::new()),
    }
}
