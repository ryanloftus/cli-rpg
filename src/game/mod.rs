mod menu;
use crate::area::story::{story_component_prompt, StoryComponent, StoryComponentAction};
use crate::area::{self, Area, AreaResult};
use crate::battle::{battle, BattleResult};
use crate::player::class::choose_class_prompt;
use crate::player::Player;
use crate::prompt::{get_selection_from_options, PromptOption};
use crate::save::{self, save};

#[derive(Debug, Clone)]
enum PlayerAction {
    EnterNextArea,
    ReturnToPreviousArea,
    QuitGame,
}

impl PromptOption for PlayerAction {
    fn option_name(&self) -> String {
        String::from(match self {
            PlayerAction::EnterNextArea => "Enter next area",
            PlayerAction::ReturnToPreviousArea => "Return to a previous area",
            PlayerAction::QuitGame => "Quit game",
        })
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(match self {
            PlayerAction::EnterNextArea => "E",
            PlayerAction::ReturnToPreviousArea => "R",
            PlayerAction::QuitGame => "Q",
        }))
    }
}

pub fn play_game() {
    let mut player: &mut Player = &mut menu::start();
    save(&player);
    let areas = area::build_areas();
    let current_area = player.story_progress.areas_completed;
    loop {
        // TODO: provide a way for the player to view their stats, class, etc
        let area_result = enter(player, &areas[current_area]);
        match area_result {
            area::AreaResult::ReturnToPreviousArea => {
                return_to_previous_area(player, &choose_previous_area(player, &areas));
            }
            area::AreaResult::AreaCompleted => {
                player.experience.area_cleared();
                player.story_progress.areas_completed += 1;
                player.story_progress.current_area_progress = 0;
                save(&player);
                if player.story_progress.areas_completed == areas.len() {
                    return;
                } else {
                    choose_class_prompt(&player.class);
                    match get_action_between_areas() {
                        PlayerAction::EnterNextArea => todo!(),
                        PlayerAction::ReturnToPreviousArea => todo!(),
                        PlayerAction::QuitGame => todo!(),
                    }
                }
            }
            area::AreaResult::PlayerWasDefeated => {
                return_to_previous_area(player, &areas[player.story_progress.areas_completed - 1]);
            }
            area::AreaResult::QuitGame => {
                return;
            }
        }
    }
}

/*
 * Starts or continues players progress in an area from the given entry point
 * entry_point is an idx in area.story
 */
fn enter(player: &mut Player, area: &Area) -> AreaResult {
    while player.story_progress.current_area_progress < area.story.len() {
        let action = get_action_in_area(area, player.story_progress.current_area_progress);
        match action {
            StoryComponentAction::ShowText(text) => {
                println!("{text}");
                player.story_progress.current_area_progress += 1;
            }
            StoryComponentAction::Battle(enemies) => {
                match battle(&player, &enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(&enemies),
                    BattleResult::Defeat => return AreaResult::PlayerWasDefeated,
                }
                player.story_progress.current_area_progress += enemies.len();
            }
            StoryComponentAction::BossBattle(boss) => {
                let enemies = vec![boss];
                match battle(&player, &enemies) {
                    BattleResult::Victory => player.experience.enemies_defeated(&enemies),
                    BattleResult::Defeat => return AreaResult::PlayerWasDefeated,
                }
                player.story_progress.current_area_progress += 1;
            }
            StoryComponentAction::ReturnToPreviousArea => return AreaResult::ReturnToPreviousArea,
            StoryComponentAction::QuitGame => return AreaResult::QuitGame,
        }
        save::save(&player);
    }
    return AreaResult::AreaCompleted;
}

/*
 * Should only be called once the player has completed this areas story
 * Player returns to the area to train by fighting practice battles
 */
fn train(player: &mut Player, area: &Area) {
    todo!("implement training");
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
            story_component_prompt::show_enemy_prompt(enemies)
        }
        StoryComponent::Boss(boss) => story_component_prompt::show_boss_prompt(&boss),
    }
}

fn get_action_between_areas() -> PlayerAction {
    return get_selection_from_options(
        String::from("What will you do next?"),
        &vec![
            PlayerAction::EnterNextArea,
            PlayerAction::ReturnToPreviousArea,
            PlayerAction::QuitGame,
        ],
    );
}

/*
 * returns the index of the area selected
 */
fn choose_previous_area(player: &Player, areas: &Vec<Area>) -> Area {
    return get_selection_from_options(
        String::from("Select an area to return to."),
        &areas[0..player.story_progress.areas_completed].to_vec(),
    );
}

fn return_to_previous_area(player: &mut Player, area: &Area) {
    train(player, area);
}
