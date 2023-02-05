pub mod attribute;
use attribute::SkillAttribute;
use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Skill {}

impl Skill {
    pub fn attributes(&self) -> Vec<SkillAttribute> {
        return vec![];
    }
}

impl PromptOption for Skill {
    fn option_name(&self) -> String {}

    fn short_option_name(&self) -> Option<String> {}
}
