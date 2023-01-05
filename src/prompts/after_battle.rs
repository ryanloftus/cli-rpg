use super::InputPrompt;
use super::PromptOption;

pub const NEXT_BATTLE_OPTION: PromptOption = PromptOption {
    name: "Next battle",
    short_name: "N",
};

pub const REST_OPTION: PromptOption = PromptOption {
    name: "Rest",
    short_name: "R",
};

pub const TRAIN_OPTION: PromptOption = PromptOption {
    name: "Train",
    short_name: "T",
};

const PROMPT: InputPrompt = InputPrompt {
    initial_prompt: "What will you do next?",
    options: &[NEXT_BATTLE_OPTION, REST_OPTION, TRAIN_OPTION],
};

pub fn show_prompt() {
    PROMPT.show();
}
