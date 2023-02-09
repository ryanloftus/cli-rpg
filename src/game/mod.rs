mod battle;
mod menu;
mod prompts;
mod story;
use self::battle::{battle, BattleResult};
use self::story::StoryComponentAction;
use crate::area::{Area, StoryComponent, NUM_AREAS};
use crate::player::class::choose_class_prompt;
use crate::player::Player;
use crate::prompt::{
    get_optional_selection_from_options, get_selection_from_options, PromptOption,
};
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
    let mut player_action = get_player_action(player);
    loop {
        match player_action {
            PlayerAction::EnterNextArea => {
                player_action = enter(player, Area::area_at(player.story_progress.areas_completed));
            }
            PlayerAction::ReturnToCurrentArea => {
                player_action = enter(player, Area::area_at(player.story_progress.areas_completed));
            }
            PlayerAction::ReturnToPreviousArea => {
                if let Some(area) = select_area_to_return_to(player) {
                    player_action = enter(player, area);
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
fn enter(player: &mut Player, area: Area) -> PlayerAction {
    let area_result = if Area::area_at(player.story_progress.areas_completed) == area {
        do_story(player, area)
    } else {
        train(player, area)
    };
    return match area_result {
        AreaResult::LeftArea => get_player_action(player),
        AreaResult::AreaCompleted => on_area_completed(player),
        AreaResult::PlayerWasDefeated => {
            println!("You allies rescued you and brought you back to The Kingdom.");
            enter(player, Area::Kingdom)
        }
    };
}

fn do_story(player: &mut Player, area: Area) -> AreaResult {
    let story = area.story();
    while player.story_progress.current_area_progress < story.len() {
        let action = get_action_in_area(&story, player.story_progress.current_area_progress);
        let progress = match action {
            StoryComponentAction::ShowText(text) => {
                println!("{text}");
                1
            }
            StoryComponentAction::ShowPlayerInfo => {
                player.print_summary();
                0
            }
            StoryComponentAction::Battle(num_enemies) => {
                let start_idx = player.story_progress.current_area_progress;
                let end_idx = player.story_progress.current_area_progress + num_enemies;
                let enemies = &story[start_idx..end_idx]
                    .iter()
                    .map(|sc| match sc {
                        StoryComponent::Enemy(enemy) => *enemy,
                        _ => panic!(),
                    })
                    .collect();
                match battle(&player, enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(enemies),
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
fn train(player: &mut Player, area: Area) -> AreaResult {
    loop {
        // TODO: maybe have a training enemy prompt
        match prompts::show_enemy_prompt(10) {
            StoryComponentAction::ShowPlayerInfo => player.print_summary(),
            StoryComponentAction::Battle(num_enemies) => {
                let enemies = &area.generate_training_enemies(num_enemies, player.experience.level);
                match battle(player, enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(enemies),
                    BattleResult::Defeat => {}
                }
            }
            StoryComponentAction::LeaveArea => return AreaResult::LeftArea,
            _ => panic!("Invalid action occurred in training"),
        }
    }
}

fn on_area_completed(player: &mut Player) -> PlayerAction {
    player.experience.area_cleared();
    player.story_progress.areas_completed += 1;
    player.story_progress.current_area_progress = 0;
    save(&player);
    if player.story_progress.areas_completed == NUM_AREAS {
        return PlayerAction::QuitGame;
    } else {
        choose_class_prompt(&player.class);
        return get_action_after_area_completed();
    }
}

fn get_action_in_area(story: &Vec<StoryComponent>, story_idx: usize) -> StoryComponentAction {
    return match story[story_idx] {
        StoryComponent::Text(text) => StoryComponentAction::ShowText(text.clone()),
        StoryComponent::Enemy(_) => {
            let mut num_enemies: usize = 0;
            for j in story_idx..story.len() {
                if let StoryComponent::Enemy(enemy) = story[j] {
                    num_enemies += 1;
                } else {
                    break;
                }
            }
            prompts::show_enemy_prompt(num_enemies)
        }
        StoryComponent::Boss(boss) => prompts::show_boss_prompt(&boss),
        StoryComponent::TutorialBattle(enemy) => prompts::show_tutorial_battle_prompt(),
    };
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

fn select_area_to_return_to(player: &mut Player) -> Option<Area> {
    let mut completed_areas = Vec::new();
    for i in 0..player.story_progress.areas_completed {
        completed_areas.push(Area::area_at(i));
    }
    return get_optional_selection_from_options(
        String::from("Select an area to return to."),
        &completed_areas,
    );
}
