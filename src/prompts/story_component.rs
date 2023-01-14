use super::InputPrompt;
use super::PromptOption;
use crate::enemy::Enemy;
use crate::utils::io_util::request_num;
use std::borrow::Cow;

pub enum StoryComponentAction {
    ShowText(String),
    Battle(u8),
    BossBattle(Enemy),
    ReturnToPreviousArea,
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

pub fn show_enemy_prompt(max_enemies: i32) -> StoryComponentAction {
    let selected_option = InputPrompt {
        initial_prompt: String::from(ENEMY_PROMPT),
        options: vec![FIGHT_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION],
    }
    .show();
    if selected_option.short_name == FIGHT_OPTION.short_name {
        let num_enemies = request_num(NUM_ENEMIES_PROMPT, 1, max_enemies) as u8;
        StoryComponentAction::Battle(num_enemies)
    } else {
        StoryComponentAction::ReturnToPreviousArea
    }
}

pub fn show_boss_prompt(boss: &Enemy) -> StoryComponentAction {
    let selected_option = InputPrompt {
        initial_prompt: format!(
            "{prompt}\nBoss: {boss_name}\nLevel: {boss_level}",
            prompt = BOSS_PROMPT,
            boss_name = boss.name,
            boss_level = boss.level,
        ),
        options: vec![FIGHT_OPTION, RETURN_TO_PREVIOUS_AREA_OPTION],
    }
    .show();
    if selected_option.short_name == FIGHT_OPTION.short_name {
        StoryComponentAction::BossBattle(boss.clone())
    } else {
        StoryComponentAction::ReturnToPreviousArea
    }
}
