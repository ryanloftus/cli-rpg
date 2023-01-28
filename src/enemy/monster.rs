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

pub fn new(monster_type: MonsterType, level: u16) -> Enemy {
    match monster_type {
        MonsterType::Slime => new_slime(level),
        MonsterType::Goblin => new_goblin(level),
        MonsterType::Dragon => new_dragon(level),
        MonsterType::Ogre => new_ogre(level),
        MonsterType::Demon => new_demon(level),
        MonsterType::DemonicBeast => new_demonic_beast(level),
    }
}

fn new_slime(level: u16) -> Enemy {
    Enemy {
        name: String::from("Slime"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats {
            max_health: todo!(),
            max_mp: todo!(),
            strength: todo!(),
            magic: todo!(),
            defense: todo!(),
            magic_resist: todo!(),
            speed: todo!(),
            skill: todo!(),
            luck: todo!(),
        },
    }
}

fn new_goblin(level: u16) -> Enemy {
    Enemy {
        name: String::from("Goblin"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats {
            max_health: todo!(),
            max_mp: todo!(),
            strength: todo!(),
            magic: todo!(),
            defense: todo!(),
            magic_resist: todo!(),
            speed: todo!(),
            skill: todo!(),
            luck: todo!(),
        },
    }
}

fn new_dragon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Dragon"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats {
            max_health: todo!(),
            max_mp: todo!(),
            strength: todo!(),
            magic: todo!(),
            defense: todo!(),
            magic_resist: todo!(),
            speed: todo!(),
            skill: todo!(),
            luck: todo!(),
        },
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
            stats: Stats {
                max_health: todo!(),
                max_mp: todo!(),
                strength: todo!(),
                magic: todo!(),
                defense: todo!(),
                magic_resist: todo!(),
                speed: todo!(),
                skill: todo!(),
                luck: todo!(),
            },
        }
    } else {
        Enemy {
            name: String::from("Ogre"),
            level,
            skills: Vec::new(),
            difficulty: super::EnemyDifficulty::Strong,
            stats: Stats {
                max_health: todo!(),
                max_mp: todo!(),
                strength: todo!(),
                magic: todo!(),
                defense: todo!(),
                magic_resist: todo!(),
                speed: todo!(),
                skill: todo!(),
                luck: todo!(),
            },
        }
    }
}

fn new_demon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demon"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats {
            max_health: todo!(),
            max_mp: todo!(),
            strength: todo!(),
            magic: todo!(),
            defense: todo!(),
            magic_resist: todo!(),
            speed: todo!(),
            skill: todo!(),
            luck: todo!(),
        },
    }
}

fn new_demonic_beast(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demonic Beast"),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats {
            max_health: todo!(),
            max_mp: todo!(),
            strength: todo!(),
            magic: todo!(),
            defense: todo!(),
            magic_resist: todo!(),
            speed: todo!(),
            skill: todo!(),
            luck: todo!(),
        },
    }
}
