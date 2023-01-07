use serde::{Deserialize, Serialize};

/*
 * SkillAttributes describe a Skill and allow Skills to have more complex effects than simply dealing damage
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillAttribute {
    name: &'static str,
}

pub const RANGED: SkillAttribute = SkillAttribute { name: "Ranged" };
pub const MELEE: SkillAttribute = SkillAttribute { name: "Melee" };
pub const MAGIC: SkillAttribute = SkillAttribute { name: "Magic" };
pub const PHYSICAL: SkillAttribute = SkillAttribute { name: "Physical" };
pub const HOT: SkillAttribute = SkillAttribute { name: "Hot" };
pub const COLD: SkillAttribute = SkillAttribute { name: "Cold" };
pub const ELECTRIC: SkillAttribute = SkillAttribute { name: "Electric" };
pub const DARK: SkillAttribute = SkillAttribute { name: "Dark" };
pub const LIGHT: SkillAttribute = SkillAttribute { name: "Light" };
pub const HEALING: SkillAttribute = SkillAttribute { name: "Healing" };
pub const DEFENSIVE: SkillAttribute = SkillAttribute { name: "Defensive" };
pub const DOT: SkillAttribute = SkillAttribute { name: "Damage Over Time" };
pub const AOE: SkillAttribute = SkillAttribute { name: "Area of Effect" };
pub const ARMOR_PEN: SkillAttribute = SkillAttribute { name: "Armor Piercing" };
pub const MAGIC_PEN: SkillAttribute = SkillAttribute { name: "Magic Piercing" };
