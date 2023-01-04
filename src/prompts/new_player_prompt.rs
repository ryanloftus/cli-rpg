use crate::utils::io_util;
use crate::player::Player;
use super::class_prompt::starting_class_prompt;

fn is_valid_name(name: String) -> bool {
    name.chars().all(char::is_alphabetic)
}

pub fn new_player() -> Player {
    let name = io_util::request_input(
        "What is your name?",
        is_valid_name,
        "Name must be alphabetic (letters only). Please enter a different name."
    );
    println!("Hello, {name}!");
    let class = starting_class_prompt();
    println!("Now that you are able to fight... The world is in need of a hero...");
    println!("Territories far and wide have all been suffering from various wars, monster attacks, and infighting.");
    let will_save_world = io_util::request_yes_or_no("Will you help save the world? [Y/N]");
    if will_save_world {
        Player::new(name, class)
    } else {
        panic!("The world was not saved.");
    }
}
