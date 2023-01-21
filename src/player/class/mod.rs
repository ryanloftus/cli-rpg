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
pub enum Class { // TODO: create functions for getting each classes class progression, create functions for getting the stat boosts, skills, and attributes that come with the class
    FutureHero,
    Starter(StarterClass),
    Intermediate(IntermediateClass),
    Advanced(AdvancedClass),
    Expert(ExpertClass),
    Master(MasterClass),
    Overpowered(OverpoweredClass),
}

impl Class {
    pub fn class_progressions(&self) -> Vec<Class> {
        match self {
            Class::FutureHero => todo!(),
            Class::Starter(_) => todo!(),
            Class::Intermediate(_) => todo!(),
            Class::Advanced(_) => todo!(),
            Class::Expert(_) => todo!(),
            Class::Master(_) => todo!(),
            Class::Overpowered(_) => panic!("Cannot progress past OverpoweredClass"),
        }
    }
}

impl PromptOption for Class {
    fn option_name(&self) -> String {
        match self {
            Class::FutureHero => String::from("Future Hero"),
            Class::Starter(starter_class) => starter_class.option_name(),
            Class::Intermediate(intermediate_class) => intermediate_class.option_name(),
            Class::Advanced(advanced_class) => advanced_class.option_name(),
            Class::Expert(expert_class) => expert_class.option_name(),
            Class::Master(master_class) => master_class.option_name(),
            Class::Overpowered(overpowered_class) => overpowered_class.option_name(),
        }
    }

    fn short_option_name(&self) -> Option<String> {
        match self {
            Class::FutureHero => Some(String::from("F")),
            Class::Starter(starter_class) => starter_class.short_option_name(),
            Class::Intermediate(intermediate_class) => intermediate_class.short_option_name(),
            Class::Advanced(advanced_class) => advanced_class.short_option_name(),
            Class::Expert(expert_class) => expert_class.short_option_name(),
            Class::Master(master_class) => master_class.short_option_name(),
            Class::Overpowered(overpowered_class) => overpowered_class.short_option_name(),
        }
    }
}

pub fn choose_class_prompt(class_options: Vec<Class>) -> Class {
    return get_selection_from_options(String::from("Choose a class."), &class_options);
}
