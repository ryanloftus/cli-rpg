use crate::stats::Stats;

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
    // TODO: make logic more complex, (guards and lieutenants should be stronger)
    Enemy {
        name: enemy_name(soldier_type, faction),
        level,
        skills: Vec::new(),
        difficulty: if soldier_type == SoldierType::Lieutenant {
            super::EnemyDifficulty::Strong
        } else {
            super::EnemyDifficulty::Weak
        },
        stats: Stats::new(level, Vec::new()),
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
