pub mod attribute;
use attribute::SkillAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub attributes: Vec<SkillAttribute>,
}
