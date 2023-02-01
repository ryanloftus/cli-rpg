use super::story::StoryComponentAction;
use crate::enemy::Enemy;
use crate::prompt::{get_selection_from_numeric_range, get_selection_from_options};

const ENEMY_PROMPT: &str = "You see enemies ahead. What will you do?";
const NUM_ENEMIES_PROMPT: &str = "How many enemies will you take on?";
const BOSS_PROMPT: &str = "You see a boss ahead. What will you do?";

pub fn show_enemy_prompt(upcoming_enemies: Vec<&Enemy>) -> StoryComponentAction {
    let selected_option = get_selection_from_options(
        String::from(ENEMY_PROMPT),
        &vec![
            StoryComponentAction::Battle(Vec::new()),
            StoryComponentAction::LeaveArea, // TODO: remove this option in tutorial
            StoryComponentAction::ShowPlayerInfo,
        ],
    );
    return match selected_option {
        StoryComponentAction::Battle(_) => {
            let num_enemies = get_selection_from_numeric_range(
                NUM_ENEMIES_PROMPT,
                1,
                upcoming_enemies.len() as i32,
            ) as usize;
            StoryComponentAction::Battle(
                upcoming_enemies[0..num_enemies]
                    .iter()
                    .map(|e| (**e).clone())
                    .collect(),
            )
        }
        StoryComponentAction::LeaveArea => StoryComponentAction::LeaveArea,
        _ => panic!("Invalid option selected from enemy battle prompt"),
    };
}

pub fn show_boss_prompt(boss: &Enemy) -> StoryComponentAction {
    get_selection_from_options(
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
    )
}
