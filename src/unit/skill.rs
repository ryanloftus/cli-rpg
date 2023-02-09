use super::experience::Experience;
use crate::attribute::Attribute;
use crate::prompt::PromptOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    skill_type: SkillType,
    experience: Experience,
    attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SkillType {}

impl PromptOption for Skill {
    fn option_name(&self) -> String {
        todo!()
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
