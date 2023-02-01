mod battle;
mod menu;
mod prompts;
mod story;
use self::battle::{battle, BattleResult};
use self::story::StoryComponentAction;
use crate::area::{self, Area, StoryComponent};
use crate::player::class::choose_class_prompt;
use crate::player::Player;
use crate::prompt::{get_selection_from_options, PromptOption};
use crate::save::{self, save};

#[derive(Debug, Clone)]
enum PlayerAction {
    EnterNextArea,
    ReturnToCurrentArea, // only one of "EnterNextArea" and "ReturnToCurrentArea" should be available at a time
    ReturnToPreviousArea,
    ShowPlayerInfo,
    QuitGame,
}

impl PromptOption for PlayerAction {
    fn option_name(&self) -> String {
        String::from(match self {
            PlayerAction::EnterNextArea => "Enter next area",
            PlayerAction::ReturnToCurrentArea => "Return to current area",
            PlayerAction::ReturnToPreviousArea => "Return to a previous area",
            PlayerAction::QuitGame => "Quit game",
            PlayerAction::ShowPlayerInfo => "Show player info",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            PlayerAction::EnterNextArea => "E",
            PlayerAction::ReturnToCurrentArea => "C",
            PlayerAction::ReturnToPreviousArea => "R",
            PlayerAction::QuitGame => "Q",
            PlayerAction::ShowPlayerInfo => "P",
        }))
    }
}

/*
 * Why is the player leaving the area
 */
enum AreaResult {
    LeftArea,
    AreaCompleted,
    PlayerWasDefeated,
}

pub fn play_game() {
    let player: &mut Player = &mut menu::start();
    save(&player);
    let areas = area::build_areas();
    let mut player_action = get_player_action(player);
    loop {
        match player_action {
            PlayerAction::EnterNextArea => {
                player_action = enter(player, &areas, player.story_progress.areas_completed);
            }
            PlayerAction::ReturnToCurrentArea => {
                player_action = enter(player, &areas, player.story_progress.areas_completed);
            }
            PlayerAction::ReturnToPreviousArea => {
                let area_idx = select_area_to_return_to(player, &areas);
                if let Some(idx) = area_idx {
                    player_action = enter(player, &areas, idx);
                } else {
                    player_action = get_player_action(player);
                }
            }
            PlayerAction::ShowPlayerInfo => {
                player.print_summary();
                player_action = get_player_action(player);
            }
            PlayerAction::QuitGame => return,
        }
    }
}

fn get_player_action(player: &Player) -> PlayerAction {
    return if player.story_progress.current_area_progress == 0 {
        get_action_after_area_completed()
    } else {
        get_action_after_area_left()
    };
}

/*
 * Starts or continues players progress in an area from the given entry point
 * entry_point is an idx in area.story
 */
fn enter(player: &mut Player, areas: &Vec<Area>, area_idx: usize) -> PlayerAction {
    let area_result = if player.story_progress.areas_completed == area_idx {
        do_story(player, &areas[area_idx])
    } else {
        train(player, &areas[area_idx])
    };
    return match area_result {
        AreaResult::LeftArea => get_action_after_area_left(),
        AreaResult::AreaCompleted => on_area_completed(player, &areas),
        AreaResult::PlayerWasDefeated => {
            println!("You allies rescued you and brought you back to The Kingdom.");
            enter(player, areas, 0)
        }
    };
}

fn do_story(player: &mut Player, area: &Area) -> AreaResult {
    while player.story_progress.current_area_progress < area.story.len() {
        let action = get_action_in_area(area, player.story_progress.current_area_progress);
        let progress = match action {
            StoryComponentAction::ShowText(text) => {
                println!("{text}");
                1
            }
            StoryComponentAction::ShowPlayerInfo => {
                player.print_summary();
                0
            }
            StoryComponentAction::Battle(enemies) => {
                match battle(&player, &enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(&enemies),
                    BattleResult::Defeat => return AreaResult::PlayerWasDefeated,
                }
                enemies.len()
            }
            StoryComponentAction::BossBattle(boss) => {
                let enemies = vec![boss];
                match battle(&player, &enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(&enemies),
                    BattleResult::Defeat => return AreaResult::PlayerWasDefeated,
                }
                1
            }
            StoryComponentAction::LeaveArea => return AreaResult::LeftArea,
        };
        player.story_progress.current_area_progress += progress;
        save::save(&player);
    }
    return AreaResult::AreaCompleted;
}

/*
 * Should only be called once the player has completed this areas story
 * Player returns to the area to train by fighting practice battles
 */
fn train(player: &mut Player, area: &Area) -> AreaResult {
    todo!("implement training");
}

fn on_area_completed(player: &mut Player, areas: &Vec<Area>) -> PlayerAction {
    player.experience.area_cleared();
    player.story_progress.areas_completed += 1;
    player.story_progress.current_area_progress = 0;
    save(&player);
    if player.story_progress.areas_completed == areas.len() {
        return PlayerAction::QuitGame;
    } else {
        choose_class_prompt(&player.class);
        return get_action_after_area_completed();
    }
}

fn get_action_in_area(area: &Area, story_idx: usize) -> StoryComponentAction {
    match &area.story[story_idx] {
        StoryComponent::Text(text) => StoryComponentAction::ShowText(text.clone()),
        StoryComponent::Enemy(_) => {
            let mut enemies = Vec::new();
            for j in story_idx..area.story.len() {
                if let StoryComponent::Enemy(enemy) = &area.story[j] {
                    enemies.push(enemy);
                }
            }
            prompts::show_enemy_prompt(enemies)
        }
        StoryComponent::Boss(boss) => prompts::show_boss_prompt(&boss),
    }
}

fn get_action_after_area_left() -> PlayerAction {
    return get_selection_from_options(
        String::from("What will you do next?"),
        &vec![
            PlayerAction::ReturnToCurrentArea,
            PlayerAction::ReturnToPreviousArea,
            PlayerAction::ShowPlayerInfo,
            PlayerAction::QuitGame,
        ],
    );
}

fn get_action_after_area_completed() -> PlayerAction {
    return get_selection_from_options(
        String::from("What will you do next?"),
        &vec![
            PlayerAction::EnterNextArea,
            PlayerAction::ReturnToPreviousArea,
            PlayerAction::ShowPlayerInfo,
            PlayerAction::QuitGame,
        ],
    );
}

fn select_area_to_return_to(player: &mut Player, areas: &Vec<Area>) -> Option<usize> {
    let area = get_selection_from_options(
        String::from("Select an area to return to."),
        &areas[0..player.story_progress.areas_completed].to_vec(),
    ); // TODO: create another function in prompt that allows the user to 'cancel', returning None
    let idx = areas.iter().position(|a| area == *a);
    return idx;
}
