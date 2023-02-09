use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

/*
 * Attributes describe a Skill or fighter and are used to determine effectiveness in battle
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
        todo!()
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
