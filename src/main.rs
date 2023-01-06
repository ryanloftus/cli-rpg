mod area;
mod class;
mod enemy;
mod player;
mod prompts;
mod skill;
mod utils;

use utils::save::save;
use prompts::start_game;

fn main() {
    let mut player = start_game::new_player();
    save(&player);
}
