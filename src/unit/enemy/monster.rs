use super::super::stats::{StatMultiplier, Stats};
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
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(level, vec![StatMultiplier::MagicResist(1.25)]),
    }
}

fn new_goblin(level: u16) -> Enemy {
    Enemy {
        name: String::from("Goblin"),
        level,
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(
            level,
            vec![
                StatMultiplier::MaxHealth(0.75),
                StatMultiplier::Strength(1.25),
                StatMultiplier::Skill(0.75),
            ],
        ),
    }
}

fn new_dragon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Dragon"),
        level,
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats::new(
            level,
            vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(1.75),
                StatMultiplier::Magic(1.25),
                StatMultiplier::Defense(2.0),
                StatMultiplier::MagicResist(1.5),
                StatMultiplier::Speed(1.25),
            ],
        ),
    }
}

const SHREK_CHANCE: u16 = 5;

fn new_ogre(level: u16) -> Enemy {
    if rand::thread_rng().gen_range(0..=100) < SHREK_CHANCE {
        Enemy {
            name: String::from("Shrek"),
            level: 100,
            difficulty: super::EnemyDifficulty::Special,
            stats: Stats::new(
                level,
                vec![
                    StatMultiplier::MaxHealth(2.0),
                    StatMultiplier::Strength(2.0),
                    StatMultiplier::Magic(2.0),
                    StatMultiplier::Defense(2.0),
                    StatMultiplier::MagicResist(2.0),
                    StatMultiplier::Speed(2.0),
                    StatMultiplier::Skill(2.0),
                    StatMultiplier::Luck(2.0),
                ],
            ),
        }
    } else {
        Enemy {
            name: String::from("Ogre"),
            level,
            difficulty: super::EnemyDifficulty::Strong,
            stats: Stats::new(
                level,
                vec![
                    StatMultiplier::MaxHealth(2.0),
                    StatMultiplier::Strength(2.0),
                    StatMultiplier::Magic(0.0),
                    StatMultiplier::Defense(2.0),
                    StatMultiplier::Skill(0.5),
                    StatMultiplier::Speed(0.5),
                ],
            ),
        }
    }
}

fn new_demon(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demon"),
        level,
        difficulty: super::EnemyDifficulty::Weak,
        stats: Stats::new(
            level,
            vec![
                StatMultiplier::MaxHealth(0.75),
                StatMultiplier::Strength(1.25),
                StatMultiplier::Speed(1.5),
            ],
        ),
    }
}

fn new_demonic_beast(level: u16) -> Enemy {
    Enemy {
        name: String::from("Demonic Beast"),
        level,
        difficulty: super::EnemyDifficulty::Strong,
        stats: Stats::new(
            level,
            vec![
                StatMultiplier::MaxHealth(2.0),
                StatMultiplier::Strength(2.0),
                StatMultiplier::Defense(1.75),
                StatMultiplier::MagicResist(1.75),
                StatMultiplier::Speed(0.25),
            ],
        ),
    }
}
