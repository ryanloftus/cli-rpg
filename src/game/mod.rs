mod start_game;
use crate::save::save;
use crate::area;

pub fn play_game() {
    let mut player = start_game::start();
    save(&player);
    let areas = area::build_areas();
    let current_area = player.story_progress.areas_completed as usize;
    areas[current_area].enter(&mut player);
    // TODO: simplify control flow so we call "open_menu" and "play_game"
    // TODO: handle between area logic
}
