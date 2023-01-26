use super::Enemy;

pub enum SoldierType {
    Footsoldier,
    Archer,
    Knight,
    Guard,
    Lieutenant,
}

pub fn new(soldier_type: SoldierType, faction: String, level: u8) -> Enemy {
    // TODO: make logic more complex, (guards and lieutenants should be stronger)
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
        SoldierType::Archer => "Archer",
        SoldierType::Knight => "Knight",
        SoldierType::Guard => "Guard",
        SoldierType::Lieutenant => "Lieutenant",
    })
}
