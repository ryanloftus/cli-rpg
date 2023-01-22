use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

/*
 * SkillAttributes describe a Skill and allow Skills to have more complex effects than simply dealing damage
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillAttribute {
    Ranged,
    Melee,
    Magic,
    Physical,
    Hot,
    Cold,
    Electric,
    Dark,
    Light,
    Healing,
    Defensive,
    DamageOverTime,
    AreaOfEffect,
    ArmorPiercing,
    MagicResistancePiercing,
}

impl PromptOption for SkillAttribute {
    fn option_name(&self) -> String {
        todo!()
    }

    fn short_option_name(&self) -> Option<String> {
        todo!()
    }
}
