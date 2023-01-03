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
        "Name must be alphetic (letters only). Please enter a different name."
    );
    println!("Hello, {name}!");
    let class = starting_class_prompt();
    Player {
        name: name,
        class: class,
        level: 1,
    }
}
