mod area;
mod class;
mod enemy;
mod player;
mod prompts;
mod skill;
mod story;
mod utils;

use utils::save;
use prompts::start_game;

fn main() {
    let mut player = start_game::new_player();
    save::create_save_file(&player);
}
