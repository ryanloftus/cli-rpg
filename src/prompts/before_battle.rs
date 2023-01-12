use super::InputPrompt;
use super::PromptOption;
use crate::utils::io_util::request_num;
use std::borrow::Cow;

pub enum BeforeBattleAction {
    Fight(u8),
    ReturnToPreviousArea(String),
}

const FIGHT_OPTION: PromptOption = PromptOption {
    name: Cow::Borrowed("Fight"),
    short_name: Some("F"),
};

const RETURN_TO_PREVIOUS_AREA_OPTION: PromptOption = PromptOption {
    name: Cow::Borrowed("Return to a previous area to train"),
    short_name: Some("R"),
};

const ENEMY_PROMPT: &str = "You see enemies ahead. What will you do?";
const NUM_ENEMIES_PROMPT: &str = "How many enemies will you take on?";
const BOSS_PROMPT: &str = "You see a boss ahead. What will you do?";

pub fn show_enemy_prompt() -> BeforeBattleAction {
    let selected_option = InputPrompt {
        initial_prompt: String::from(ENEMY_PROMPT),
        options: vec![FIGHT_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION],
    }.show();
    if selected_option.short_name == FIGHT_OPTION.short_name {
        let num_enemies = request_num(
            NUM_ENEMIES_PROMPT,
            u8::MIN as i32,
            u8::MAX as i32,
        ) as u8;
        BeforeBattleAction::Fight(num_enemies)
    } else {
        let area_to_return_to = 
        BeforeBattleAction::ReturnToPreviousArea
    }
}

pub fn show_boss_prompt() -> BeforeBattleAction {
    let selected_option = InputPrompt {
        initial_prompt: String::from(ENEMY_PROMPT),
        options: vec![FIGHT_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION],
    }.show();
    if selected_option.short_name == FIGHT_OPTION.short_name {
        let num_enemies = request_num(
            NUM_ENEMIES_PROMPT,
            u8::MIN as i32,
            u8::MAX as i32,
        ) as u8;
        BeforeBattleAction::Fight(num_enemies)
    } else {
        BeforeBattleAction::ReturnToPreviousArea
    }
}
