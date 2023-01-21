use serde::{Serialize, Deserialize};

use crate::prompt::PromptOption;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExpertClass {
    ExplosionExpert,
}

impl PromptOption for ExpertClass {
    fn option_name(&self) -> String {
        String::from(
            match self {
                ExpertClass::ExplosionExpert => "Explosion Expert",
            }
        )
    }

    fn short_option_name(&self) -> Option<String> {
        Some(
            String::from(
                match self {
                    ExpertClass::ExplosionExpert => "E",
                }
            )
        )
    }
}
