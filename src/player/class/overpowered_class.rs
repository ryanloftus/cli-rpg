use serde::{Deserialize, Serialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OverpoweredClass {
    GodOfExplosions,
    DemonLord,
    ArchitectOfChaos,
    // TODO: add more
}

impl PromptOption for OverpoweredClass {
    fn option_name(&self) -> String {
        String::from(match self {
            OverpoweredClass::GodOfExplosions => "God of Explosions",
            OverpoweredClass::DemonLord => "Demon Lord",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            OverpoweredClass::GodOfExplosions => "E",
            OverpoweredClass::DemonLord => "D",
        }))
    }
}
