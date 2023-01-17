pub mod attribute;
use attribute::SkillAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub attributes: Vec<SkillAttribute>,
}
