pub mod story_component_prompt;

use crate::{enemy::Enemy, prompt::PromptOption};

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Text(String),
    Enemy(Enemy),
    Boss(Enemy),
    // TODO: GainAttribute once player attributes are added
    // optional todo: implement prompts inside area stories and make stories dynamic (change based on prompt responses)
}

#[derive(Debug, Clone)]
pub enum StoryComponentAction {
    ShowText(String),
    Battle(Vec<Enemy>),
    BossBattle(Enemy),
    ReturnToPreviousArea,
}

impl PromptOption for StoryComponentAction {
    fn option_name(&self) -> String {
        String::from(match self {
            StoryComponentAction::Battle(_) => "Fight",
            StoryComponentAction::BossBattle(_) => "Boss",
            StoryComponentAction::ReturnToPreviousArea => "Return to a previous area to train",
            _ => panic!(),
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            StoryComponentAction::Battle(_) => "F",
            StoryComponentAction::BossBattle(_) => "B",
            StoryComponentAction::ReturnToPreviousArea => "R",
            _ => panic!(),
        }))
    }
}
