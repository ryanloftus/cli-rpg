use super::class::starting_class_prompt;
use crate::player::Player;
use crate::utils::io_util;

const WILL_SAVE_WORLD_PROMPT: &str = "Now that you are able to fight... The world is in need of a hero...
Territories far and wide have all been suffering from wars, monster attacks, and natural disasters.
Will you help save the world? [Y/N]";

fn is_valid_name(name: String) -> bool {
    name.chars().all(char::is_alphabetic)
}

pub fn new_player() -> Player {
    let name = io_util::request_input_with_validation(
        "What is your name?",
        is_valid_name,
        "Name must be alphabetic (letters only). Please enter a different name.",
    );
    println!("Hello, {name}!");
    let class = starting_class_prompt();
    let will_save_world = io_util::request_yes_or_no(WILL_SAVE_WORLD_PROMPT);
    if will_save_world {
        Player::new(name, class)
    } else {
        panic!("The world was not saved.");
    }
}
