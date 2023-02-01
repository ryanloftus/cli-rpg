use crate::{enemy::Enemy, prompt::PromptOption};

#[derive(Debug, Clone)]
pub enum StoryComponentAction {
    ShowText(String),
    ShowPlayerInfo,
    Battle(Vec<Enemy>),
    BossBattle(Enemy),
    ReturnToPreviousArea,
    QuitGame,
}

impl PromptOption for StoryComponentAction {
    fn option_name(&self) -> String {
        String::from(match self {
            StoryComponentAction::Battle(_) => "Fight",
            StoryComponentAction::ShowPlayerInfo => "Show player info",
            StoryComponentAction::BossBattle(_) => "Boss",
            StoryComponentAction::ReturnToPreviousArea => "Return to a previous area to train",
            StoryComponentAction::QuitGame => "Quit game",
            _ => panic!(),
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            StoryComponentAction::ShowPlayerInfo => "P",
            StoryComponentAction::Battle(_) => "F",
            StoryComponentAction::BossBattle(_) => "B",
            StoryComponentAction::ReturnToPreviousArea => "R",
            StoryComponentAction::QuitGame => "Q",
            _ => panic!(),
        }))
    }
}
