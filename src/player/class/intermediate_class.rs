use crate::prompt::PromptOption;

#[derive(Debug, Clone)]
pub enum IntermediateClass {
    ExplosiveMage,
    // TODO: add Intermediate Classes
}

impl PromptOption for IntermediateClass {
    fn option_name(&self) -> String {
        String::from(
            match self {
                IntermediateClass::ExplosiveMage => "Explosive Mage",
            }
        )
    }

    fn short_option_name(&self) -> Option<String> {
        Some(
            String::from(
                match self {
                    IntermediateClass::ExplosiveMage => "E",
                }
            )
        )
    }
}
