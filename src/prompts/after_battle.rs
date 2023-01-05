use super::InputPrompt;
use super::PromptOption;

pub enum AfterBattleAction {
    NextBattle,
    ReturnToPreviousArea,
}

const NEXT_BATTLE_OPTION: PromptOption = PromptOption {
    name: "Next battle",
    short_name: "N",
};

const RETURN_TO_PREVIOUS_AREA_OPTION: PromptOption = PromptOption {
    name: "Return to a previous area",
    short_name: "R",
};

const PROMPT: InputPrompt = InputPrompt {
    initial_prompt: "What will you do next?",
    options: &[NEXT_BATTLE_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION],
};

pub fn prompt() -> AfterBattleAction {
    let selected_option = PROMPT.show();
    if selected_option.short_name == NEXT_BATTLE_OPTION.short_name {
        AfterBattleAction::NextBattle
    } else {
        AfterBattleAction::ReturnToPreviousArea
    }
}
