mod start_game;
use crate::prompt::get_selection_from_options;
use crate::save::save;
use crate::area;

const RETURN_TO_AREA_PROMPT: &str = "Which area would you like to return to?";

pub fn play_game() {
    let mut player = start_game::start();
    save(&player);
    let areas = area::build_areas();
    let current_area = player.story_progress.areas_completed as usize;
    loop {
        let area_result = areas[current_area].enter(&mut player);
        match area_result {
            area::AreaResult::ReturnToPreviousArea => {
                let prev_area = get_selection_from_options(
                    String::from(RETURN_TO_AREA_PROMPT),
                    &areas[0..player.story_progress.areas_completed].to_vec(),
                );
                prev_area.train(&mut player);
            },
            area::AreaResult::AreaCompleted => {
                player.experience.area_cleared();
                player.story_progress.areas_completed += 1;
                player.story_progress.current_area_progress = 0;
                save(&player);
                // TODO: ask player whether they want to continue on to next area or train in a completed area
            },
            area::AreaResult::PlayerWasDefeated => {
                // TODO: player wakes up in previous area, no progress is lost
                
            },
        }
    }
    // TODO: simplify control flow so we call "open_menu" and "play_game" (should there be a menu module)
    // TODO: handle between area logic
    // TODO: create a way to quit game when between areas or battles
}
