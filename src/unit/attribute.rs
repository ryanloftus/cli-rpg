use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

/*
 * Attributes describe a Skill or unit and are used to determine effectiveness in battle
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Attribute {
    Magic,
    Physical,
    Fire,
    Ice,
    Electric,
    Water,
    Wind,
    Dark,
    Light,
    Healing,
    Defensive,
    MagicResistive,
    DamageOverTime,
    AreaOfEffect,
    ArmorPiercing,
    MultiHit(u16),
}

impl PromptOption for Attribute {
    fn option_name(&self) -> String {
        return String::from(match self {
            Attribute::Magic => "Magic",
            Attribute::Physical => "Physical",
            Attribute::Fire => "Fire",
            Attribute::Ice => "Ice",
            Attribute::Electric => "Electric",
            Attribute::Water => "Water",
            Attribute::Wind => "Wind",
            Attribute::Dark => "Dark",
            Attribute::Light => "Light",
            Attribute::Healing => "Healing",
            Attribute::Defensive => "Defensive",
            Attribute::MagicResistive => "Magic Resistive",
            Attribute::DamageOverTime => "Damage Over Time",
            Attribute::AreaOfEffect => "Area of Effect",
            Attribute::ArmorPiercing => "Armor Piercing",
            Attribute::MultiHit(_) => "Multi-Hit",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
