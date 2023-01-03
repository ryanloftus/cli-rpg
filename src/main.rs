mod utils;
mod prompts;
mod class;
mod player;

use utils::save;
use prompts::new_player_prompt;

fn main() {
    let player = new_player_prompt::new_player();
    save::create_save_file(player.name);
}
