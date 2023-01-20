pub mod story_component_prompt;

use crate::{enemy::Enemy, prompt::PromptOption, skill::Skill};

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Enemy(Enemy),
    Boss(Enemy),
    Text(String),
    LearnSkill(&'static Skill),
    // TODO: GainAttribute once player attributes are added
    // TODO: add a Prompt variant to allow so the player can choose their own path
}

#[derive(Debug, Clone)]
pub enum StoryComponentAction {
    ShowText(String),
    Battle(Vec<Enemy>),
    BossBattle(Enemy),
    ReturnToPreviousArea, // TODO: disable this option when there is no previous area to return to (ie you are in the first area)
    LearnSkill(&'static Skill),
}

impl PromptOption for StoryComponentAction {
    fn option_name(&self) -> String {
        match self {
            StoryComponentAction::Battle(_) => String::from("Fight"),
            StoryComponentAction::BossBattle(_) => String::from("Boss"),
            StoryComponentAction::ReturnToPreviousArea => String::from("Return to a previous area to train"),
            _ => panic!("Prompt should not be shown for LearnSkill StoryComponent"),
        }
    }

    fn short_option_name(&self) -> Option<String> {
        match self {
            StoryComponentAction::Battle(_) => Some(String::from("F")),
            StoryComponentAction::BossBattle(_) => Some(String::from("B")),
            StoryComponentAction::ReturnToPreviousArea => Some(String::from("R")),
            _ => panic!("Prompt should not be shown for this StoryComponent"),
        }
    }
}
