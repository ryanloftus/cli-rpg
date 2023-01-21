use serde::{Serialize, Deserialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverpoweredClass {
    GodOfExplosions,
    // TODO: add more
}

impl PromptOption for OverpoweredClass {
    fn option_name(&self) -> String {
        String::from(
            match self {
                OverpoweredClass::GodOfExplosions => "God of Explosions",
            }
        )
    }

    fn short_option_name(&self) -> Option<String> {
        Some(
            String::from(
                match self {
                    OverpoweredClass::GodOfExplosions => "E",
                }
            )
        )
    }
}
