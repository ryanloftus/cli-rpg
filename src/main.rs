mod area;
mod class;
mod enemy;
mod player;
mod prompts;
mod skill;
mod story;
mod utils;

use utils::save;
use prompts::new_player_prompt;

fn main() {
    let mut player = new_player_prompt::new_player();
    save::create_save_file(&player);
}
