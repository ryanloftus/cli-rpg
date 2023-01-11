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
    areas[usize::from(player.story_progress.areas_completed)]
        .enter(&player, player.story_progress.current_area_progress);
}
