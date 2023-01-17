/*
 * SkillAttributes describe a Skill and allow Skills to have more complex effects than simply dealing damage
 */
pub type SkillAttribute = &'static str;

pub const RANGED: SkillAttribute = "Ranged";
pub const MELEE: SkillAttribute = "Melee";
pub const MAGIC: SkillAttribute = "Magic";
pub const PHYSICAL: SkillAttribute = "Physical";
pub const HOT: SkillAttribute = "Hot";
pub const COLD: SkillAttribute = "Cold";
pub const ELECTRIC: SkillAttribute = "Electric";
pub const DARK: SkillAttribute = "Dark";
pub const LIGHT: SkillAttribute = "Light";
pub const HEALING: SkillAttribute = "Healing";
pub const DEFENSIVE: SkillAttribute = "Defensive";
pub const DOT: SkillAttribute = "Damage Over Time";
pub const AOE: SkillAttribute = "Area of Effect";
pub const ARMOR_PEN: SkillAttribute = "Armor Piercing";
pub const MAGIC_PEN: SkillAttribute = "Magic Piercing";
