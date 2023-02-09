use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

/*
 * Attributes describe a Skill or unit and are used to determine effectiveness in battle
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    Ranged,
    Melee,
    Magic,
    Physical,
    Fire,
    Ice,
    Electric,
    Water,
    Dark,
    Light,
    Healing,
    Defensive,
    MagicResistive,
    DamageOverTime,
    AreaOfEffect,
    ArmorPiercing,
    MagicResistancePiercing,
}

impl PromptOption for Attribute {
    fn option_name(&self) -> String {
        return String::from(match self {
            Attribute::Ranged => "Ranged",
            Attribute::Melee => "Melee",
            Attribute::Magic => "Magic",
            Attribute::Physical => "Physical",
            Attribute::Fire => "Fire",
            Attribute::Ice => "Ice",
            Attribute::Electric => "Electric",
            Attribute::Water => "Water",
            Attribute::Dark => "Dark",
            Attribute::Light => "Light",
            Attribute::Healing => "Healing",
            Attribute::Defensive => "Defensive",
            Attribute::MagicResistive => "Magic Resistive",
            Attribute::DamageOverTime => "Damage Over Time",
            Attribute::AreaOfEffect => "Area of Effect",
            Attribute::ArmorPiercing => "Armor Piercing",
            Attribute::MagicResistancePiercing => "Magic Resist Piercing",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
