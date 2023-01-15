mod area;
mod battle;
mod class;
mod enemy;
mod player;
mod prompts;
mod skill;
mod utils;

use utils::save::save;

fn main() {
    let mut player = prompts::start_game::start();
    save(&player);
    let areas = area::build_areas();
    let current_area = player.story_progress.areas_completed as usize;
    let current_progress = player.story_progress.current_area_progress;
    areas[current_area].enter(&mut player, current_progress);
    // TODO: add a wrapper function "start_game" that calls "open_menu" and "play_game"
        // fn is called by main and handles between area logic
}
