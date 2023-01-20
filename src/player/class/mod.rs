pub mod starter_class;
pub mod intermediate_class;
pub mod advanced_class;
pub mod expert_class;
pub mod master_class;
pub mod overpowered_class;

use serde::{Serialize, Deserialize};

use crate::prompt::{PromptOption, get_selection_from_options};

use self::{starter_class::StarterClass, intermediate_class::IntermediateClass, advanced_class::AdvancedClass, expert_class::ExpertClass, master_class::MasterClass, overpowered_class::OverpoweredClass};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Class {
    FutureHero,
    Starter(StarterClass),
    Intermediate(IntermediateClass),
    Advanced(AdvancedClass),
    Expert(ExpertClass),
    Master(MasterClass),
    Overpowered(OverpoweredClass),
}

impl PromptOption for Class {
    fn option_name(&self) -> String {
        match self {
            Class::FutureHero => String::from("Future Hero"),
            Class::Starter(_) => todo!(),
            Class::Intermediate(_) => todo!(),
            Class::Advanced(_) => todo!(),
            Class::Expert(_) => todo!(),
            Class::Master(_) => todo!(),
            Class::Overpowered(_) => todo!(),
        }
    }

    fn short_option_name(&self) -> Option<String> {
        match self {
            Class::FutureHero => Some(String::from("F")),
            Class::Starter(_) => todo!(),
            Class::Intermediate(_) => todo!(),
            Class::Advanced(_) => todo!(),
            Class::Expert(_) => todo!(),
            Class::Master(_) => todo!(),
            Class::Overpowered(_) => todo!(),
        }
    }
}

pub fn choose_class_prompt(class_options: Vec<Class>) -> Class {
    return get_selection_from_options(String::from("Choose a class"), &class_options);
}
