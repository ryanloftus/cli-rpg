pub mod attribute;
use attribute::SkillAttribute;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    name: &'static str,
    unique_id: &'static str,
    attributes: Vec<SkillAttribute>,
}
