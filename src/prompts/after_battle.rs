use super::InputPrompt;
use super::PromptOption;
use std::borrow::Cow;

pub enum AfterBattleAction {
    NextBattle,
    ReturnToPreviousArea,
}

const NEXT_BATTLE_OPTION: PromptOption = PromptOption {
    name: Cow::Borrowed("Next battle"),
    short_name: Some("N"),
};

const RETURN_TO_PREVIOUS_AREA_OPTION: PromptOption = PromptOption {
    name: Cow::Borrowed("Return to a previous area"),
    short_name: Some("R"),
};

const PROMPT: &str = "What will you do next?";

pub fn prompt() -> AfterBattleAction {
    let selected_option = InputPrompt {
        initial_prompt: PROMPT,
        options: [NEXT_BATTLE_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION].to_vec(),
    }.show();
    if selected_option.short_name == NEXT_BATTLE_OPTION.short_name {
        AfterBattleAction::NextBattle
    } else {
        AfterBattleAction::ReturnToPreviousArea
    }
}
