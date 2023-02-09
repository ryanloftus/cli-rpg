use super::super::stats::{StatMultiplier, Stats};
use super::Enemy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoldierType {
    Footsoldier,
    Archer,
    Knight,
    Guard,
    Lieutenant,
}

pub fn new(soldier_type: SoldierType, faction: String, level: u16) -> Enemy {
    Enemy {
        name: enemy_name(soldier_type, faction),
        level,
        skills: Vec::new(),
        difficulty: if soldier_type == SoldierType::Lieutenant {
            super::EnemyDifficulty::Strong
        } else {
            super::EnemyDifficulty::Weak
        },
        stats: Stats::new(level, get_stat_mult(soldier_type)),
    }
}

fn enemy_name(soldier_type: SoldierType, faction: String) -> String {
    format!(
        "{faction} {soldier_type}",
        faction = faction,
        soldier_type = soldier_type_to_string(soldier_type),
    )
}

fn soldier_type_to_string(soldier_type: SoldierType) -> String {
    String::from(match soldier_type {
        SoldierType::Footsoldier => "Footsoldier",
        SoldierType::Archer => "Archer",
        SoldierType::Knight => "Knight",
        SoldierType::Guard => "Guard",
        SoldierType::Lieutenant => "Lieutenant",
    })
}

fn get_stat_mult(soldier_type: SoldierType) -> Vec<StatMultiplier> {
    return match soldier_type {
        SoldierType::Footsoldier => vec![StatMultiplier::Magic(0.0)],
        SoldierType::Archer => vec![
            StatMultiplier::MaxHealth(0.5),
            StatMultiplier::Defense(0.5),
            StatMultiplier::Skill(2.0),
            StatMultiplier::Speed(1.5),
            StatMultiplier::Magic(0.0),
        ],
        SoldierType::Knight => vec![
            StatMultiplier::Strength(1.5),
            StatMultiplier::Defense(1.5),
            StatMultiplier::Magic(0.0),
        ],
        SoldierType::Guard => vec![
            StatMultiplier::MaxHealth(1.75),
            StatMultiplier::Strength(1.5),
            StatMultiplier::Defense(2.0),
            StatMultiplier::MagicResist(1.5),
            StatMultiplier::Speed(0.75),
            StatMultiplier::Magic(0.0),
        ],
        SoldierType::Lieutenant => vec![
            StatMultiplier::MaxHealth(2.0),
            StatMultiplier::Strength(1.75),
            StatMultiplier::Skill(1.5),
            StatMultiplier::Defense(1.5),
            StatMultiplier::MagicResist(1.5),
            StatMultiplier::Magic(0.0),
            StatMultiplier::Luck(1.25),
        ],
    };
}
