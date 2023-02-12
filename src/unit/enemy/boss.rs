use crate::unit::stats::StatMultiplier;

use super::Enemy;

/// The boss at the end of The Plains
pub fn villainous_mage() -> Enemy {
    return Enemy::new_boss(
        String::from("Villainous Mage"),
        15,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Magic(2.0),
            StatMultiplier::Defense(0.75),
            StatMultiplier::MagicResist(2.0),
        ],
    );
}

/// The boss at the end of The Forest
pub fn demon_lord() -> Enemy {
    return Enemy::new_boss(
        String::from("Demon Lord"),
        25,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Magic(2.0),
            StatMultiplier::MagicResist(2.0),
            StatMultiplier::Speed(1.5),
            StatMultiplier::Skill(2.0),
            StatMultiplier::Luck(1.5),
        ],
    );
}

/// The warm island boss from The Islands
pub fn warm_island_king() -> Enemy {
    return Enemy::new_boss(
        String::from("Warm Island King"),
        35,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Strength(1.25),
            StatMultiplier::Magic(2.0),
            StatMultiplier::Defense(1.25),
            StatMultiplier::MagicResist(1.5),
        ],
    );
}

/// The cold island boss from The Islands
pub fn cold_island_king() -> Enemy {
    return Enemy::new_boss(
        String::from("Cold Island King"),
        40,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Strength(1.25),
            StatMultiplier::Magic(2.0),
            StatMultiplier::Defense(1.25),
            StatMultiplier::MagicResist(1.5),
        ],
    );
}

/// The stormy island boss from The Islands
pub fn stormy_island_queen() -> Enemy {
    return Enemy::new_boss(
        String::from("Stormy Island Queen"),
        45,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Strength(1.25),
            StatMultiplier::Magic(2.0),
            StatMultiplier::Defense(1.25),
            StatMultiplier::MagicResist(1.5),
        ],
    );
}

/// The boss at the end of The Caves
pub fn lizard_king() -> Enemy {
    return Enemy::new_boss(
        String::from("Lizard King"),
        60,
        vec![
            StatMultiplier::MaxHealth(5.0),
            StatMultiplier::Strength(2.0),
            StatMultiplier::Defense(1.5),
            StatMultiplier::Luck(1.5),
        ],
    );
}

/// The boss at the end of The Icefields
pub fn molten_monstrosity() -> Enemy {
    return Enemy::new_boss(
        String::from("Molten Monstrosity"),
        75,
        vec![
            StatMultiplier::MaxHealth(4.0),
            StatMultiplier::Strength(2.5),
            StatMultiplier::Defense(3.0),
            StatMultiplier::Speed(0.25),
        ],
    );
}

// The boss at the end of The Mountains (final boss)
pub fn doom_incarnate() -> Enemy {
    return Enemy::new_boss(String::from("Doom Incarnate"), 100, Vec::new());
}
