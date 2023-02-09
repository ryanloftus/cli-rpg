use super::story::StoryComponentAction;
use crate::prompt::{get_selection_from_numeric_range, get_selection_from_options};
use crate::unit::enemy::Enemy;

const TRAINING_BATTLE_PROMPT: &str = "You see a knight in the training grounds. What will you do?";
const ENEMY_PROMPT: &str = "You see enemies ahead. What will you do?";
const NUM_ENEMIES_PROMPT: &str = "How many enemies will you take on?";
const BOSS_PROMPT: &str = "You see a boss ahead. What will you do?";

const MAX_TRAINING_ENEMIES: usize = 10;

pub fn show_tutorial_battle_prompt() -> StoryComponentAction {
    return get_selection_from_options(
        String::from(TRAINING_BATTLE_PROMPT),
        &vec![StoryComponentAction::Battle(1)],
    );
}

pub fn show_training_battle_prompt() -> StoryComponentAction {
    return get_story_component_action_for_battle(TRAINING_BATTLE_PROMPT, MAX_TRAINING_ENEMIES);
}

pub fn show_enemy_prompt(max_enemies: usize) -> StoryComponentAction {
    return get_story_component_action_for_battle(ENEMY_PROMPT, max_enemies);
}

pub fn show_boss_prompt(boss: &Enemy) -> StoryComponentAction {
    return get_selection_from_options(
        format!(
            "{prompt}\nBoss: {boss_name}\nLevel: {boss_level}",
            prompt = BOSS_PROMPT,
            boss_name = boss.name,
            boss_level = boss.level,
        ),
        &vec![
            StoryComponentAction::BossBattle(boss.clone()),
            StoryComponentAction::LeaveArea,
            StoryComponentAction::ShowPlayerInfo,
        ],
    );
}

fn get_story_component_action_for_battle(prompt: &str, max_enemies: usize) -> StoryComponentAction {
    let selected_option = get_selection_from_options(
        String::from(prompt),
        &vec![
            StoryComponentAction::Battle(max_enemies),
            StoryComponentAction::LeaveArea,
            StoryComponentAction::ShowPlayerInfo,
        ],
    );
    return match selected_option {
        StoryComponentAction::Battle(_) => {
            let num_enemies =
                get_selection_from_numeric_range(NUM_ENEMIES_PROMPT, 1, max_enemies as i32)
                    as usize;
            StoryComponentAction::Battle(num_enemies)
        }
        StoryComponentAction::LeaveArea => StoryComponentAction::LeaveArea,
        StoryComponentAction::ShowPlayerInfo => StoryComponentAction::ShowPlayerInfo,
        _ => panic!("Invalid option selected from battle prompt"),
    };
}
