use super::Enemy;

pub enum SoldierType {
    Footsoldier,
    Knight,
    Archer,
    Guard,
}

pub fn new(soldier_type: SoldierType, faction: String, level: u8) -> Enemy {
    Enemy {
        name: enemy_name(soldier_type, faction),
        level,
        skills: Vec::new(),
        difficulty: super::EnemyDifficulty::Weak,
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
        SoldierType::Knight => "Knight",
        SoldierType::Archer => "Archer",
        SoldierType::Guard => "Guard",
    })
}
