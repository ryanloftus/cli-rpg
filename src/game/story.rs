use crate::{enemy::Enemy, prompt::PromptOption};

#[derive(Debug, Clone)]
pub enum StoryComponentAction {
    ShowText(String),
    ShowPlayerInfo,
    Battle(usize),
    BossBattle(Enemy),
    LeaveArea,
}

impl PromptOption for StoryComponentAction {
    fn option_name(&self) -> String {
        String::from(match self {
            StoryComponentAction::Battle(_) => "Fight",
            StoryComponentAction::ShowPlayerInfo => "Show player info",
            StoryComponentAction::BossBattle(_) => "Boss",
            StoryComponentAction::LeaveArea => "Leave the current area",
            _ => panic!(),
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            StoryComponentAction::ShowPlayerInfo => "P",
            StoryComponentAction::Battle(_) => "F",
            StoryComponentAction::BossBattle(_) => "B",
            StoryComponentAction::LeaveArea => "L",
            _ => panic!(),
        }))
    }
}
