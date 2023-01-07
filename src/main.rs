mod area;
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
}
