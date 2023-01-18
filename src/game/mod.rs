use crate::save::save;
use crate::area;
use crate::prompt;

pub fn play_game() {
    let mut player = prompt::start_game::start();
    save(&player);
    let areas = area::build_areas();
    let current_area = player.story_progress.areas_completed as usize;
    let current_progress = player.story_progress.current_area_progress;
    areas[current_area].enter(&mut player, current_progress);
    // TODO: simplify control flow so we call "open_menu" and "play_game"
    // TODO: handle between area logic
}
