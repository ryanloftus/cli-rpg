use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AdvancedClass {
    ExplosiveMage,
}

impl PromptOption for AdvancedClass {
    fn option_name(&self) -> String {
        String::from(match self {
            AdvancedClass::ExplosiveMage => "Explosive Mage",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            AdvancedClass::ExplosiveMage => "E",
        }))
    }
}
