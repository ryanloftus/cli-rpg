use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MasterClass {
    MasterOfExplosions,
    // TODO: add more
}

impl PromptOption for MasterClass {
    fn option_name(&self) -> String {
        String::from(match self {
            MasterClass::MasterOfExplosions => "Master of Explosions",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            MasterClass::MasterOfExplosions => "M",
        }))
    }
}
