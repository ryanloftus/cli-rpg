pub mod advanced_class;
pub mod expert_class;
pub mod intermediate_class;
pub mod master_class;
pub mod overpowered_class;
pub mod starter_class;

use serde::{Deserialize, Serialize};

use crate::{
    prompt::{self, PromptOption},
    unit::{
        skill::{Skill, SkillType},
        stats::StatMultiplier,
    },
};

use self::{
    advanced_class::AdvancedClass, expert_class::ExpertClass,
    intermediate_class::IntermediateClass, master_class::MasterClass,
    overpowered_class::OverpoweredClass, starter_class::StarterClass,
};

use super::Player;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Class {
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

    pub fn progressions(&self) -> Vec<Self> {
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

    pub fn stat_gain_multipliers(&self) -> Vec<StatMultiplier> {
        return match self {
            Class::FutureHero => Vec::new(),
            Class::Starter(starter_class) => starter_class.stat_gain_multipliers(),
            Class::Intermediate(intermediate_class) => intermediate_class.stat_gain_multipliers(),
            Class::Advanced(advanced_class) => advanced_class.stat_gain_multipliers(),
            Class::Expert(expert_class) => expert_class.stat_gain_multipliers(),
            Class::Master(master_class) => master_class.stat_gain_multipliers(),
            Class::Overpowered(overpowered_class) => overpowered_class.stat_gain_multipliers(),
        };
    }

    pub fn skills(&self) -> Vec<SkillType> {
        return match self {
            Class::FutureHero => Vec::new(),
            Class::Starter(starter_class) => starter_class.skills(),
            Class::Intermediate(intermediate_class) => intermediate_class.skills(),
            Class::Advanced(advanced_class) => advanced_class.skills(),
            Class::Expert(expert_class) => expert_class.skills(),
            Class::Master(master_class) => master_class.skills(),
            Class::Overpowered(overpowered_class) => overpowered_class.skills(),
        };
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

pub fn progress_class(player: &mut Player) -> Class {
    let class_options = player.class.progressions();
    let mut new_class = class_options[0];

    if class_options.len() > 1 {
        loop {
            new_class =
                prompt::get_selection_from_options(String::from("Choose a class."), &class_options);
            println!("{}", new_class.description());
            if prompt::get_boolean_selection(&format!(
                "Change to {}? [Y/N]",
                new_class.option_name()
            )) {
                return new_class;
            }
        }
    }

    add_class_skills(player, new_class);
    println!(
        "You were advanced to the {} class!",
        new_class.option_name()
    );
    return new_class;
}

fn add_class_skills(player: &mut Player, new_class: Class) {
    'outer: for new_skill in new_class.skills() {
        for i in 0..player.skills.len() {
            if player.skills[i].skill_type == new_skill {
                player.skills[i].experience.skill_evolved();
                continue 'outer;
            }
        }
        player.skills.push(Skill::new(new_skill));
    }
}
