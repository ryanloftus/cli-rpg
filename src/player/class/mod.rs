pub mod advanced_class;
pub mod expert_class;
pub mod intermediate_class;
pub mod master_class;
pub mod overpowered_class;
pub mod starter_class;

use serde::{Deserialize, Serialize};

use crate::prompt::{self, PromptOption};

use self::{
    advanced_class::AdvancedClass, expert_class::ExpertClass,
    intermediate_class::IntermediateClass, master_class::MasterClass,
    overpowered_class::OverpoweredClass, starter_class::StarterClass,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Class {
    // TODO: create functions for getting stat boosts, skills, and attributes that come with the class
    FutureHero,
    Starter(StarterClass),
    Intermediate(IntermediateClass),
    Advanced(AdvancedClass),
    Expert(ExpertClass),
    Master(MasterClass),
    Overpowered(OverpoweredClass),
}

impl Class {
    pub fn description(&self) -> String {
        return match self {
            Class::FutureHero => {
                String::from("A young individual hoping to save the world from tragedy.")
            }
            Class::Starter(starter_class) => starter_class.description(),
            Class::Intermediate(intermediate_class) => intermediate_class.description(),
            Class::Advanced(advanced_class) => advanced_class.description(),
            Class::Expert(expert_class) => expert_class.description(),
            Class::Master(master_class) => master_class.description(),
            Class::Overpowered(overpowered_class) => overpowered_class.description(),
        };
    }

    pub fn get_progressions(&self) -> Vec<Self> {
        match self {
            Class::FutureHero => vec![
                Class::Starter(StarterClass::Swordsman),
                Class::Starter(StarterClass::Knight),
                Class::Starter(StarterClass::Mage),
            ],
            Class::Starter(starter_class) => starter_class
                .progressions()
                .iter()
                .map(|c| Class::Intermediate(*c))
                .collect(),
            Class::Intermediate(intermediate_class) => intermediate_class
                .progressions()
                .iter()
                .map(|c| Class::Advanced(*c))
                .collect(),
            Class::Advanced(advanced_class) => advanced_class
                .progressions()
                .iter()
                .map(|c| Class::Expert(*c))
                .collect(),
            Class::Expert(expert_class) => expert_class
                .progressions()
                .iter()
                .map(|c| Class::Master(*c))
                .collect(),
            Class::Master(master_class) => master_class
                .progressions()
                .iter()
                .map(|c| Class::Overpowered(*c))
                .collect(),
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

pub fn choose_class_prompt(current_class: &Class) -> Class {
    let class_options = current_class.get_progressions();
    if class_options.len() == 1 {
        println!(
            "You were advanced to the {} class!",
            class_options[0].option_name()
        );
        return class_options[0].clone();
    }
    loop {
        let selected =
            prompt::get_selection_from_options(String::from("Choose a class."), &class_options);
        println!("{}", selected.description());
        if prompt::get_boolean_selection(&format!("Change to {}? [Y/N]", selected.option_name())) {
            return selected;
        }
    }
}
