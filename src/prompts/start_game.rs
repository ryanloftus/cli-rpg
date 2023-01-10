use super::class::starting_class_prompt;
use crate::player::Player;
use crate::utils::{ io_util, save };

const WILL_SAVE_WORLD_PROMPT: &str = "Now that you are able to fight... The world is in need of a hero...
Territories far and wide have all been suffering from wars, monster attacks, and natural disasters.
Will you help save the world? [Y/N]";

enum SaveFileSelection {
    ExistingSave(String),
    NewSave,
}

fn is_valid_name(name: String) -> bool {
    name.chars().all(char::is_alphabetic)
}

fn create_new_save() -> Player {
    let name = io_util::request_input_with_validation(
        "What is your name?",
        is_valid_name,
        "Name must be alphabetic (letters only). Please enter a different name.",
    );
    println!("Hello, {name}!");
    let class = starting_class_prompt();
    let will_save_world = io_util::request_yes_or_no(WILL_SAVE_WORLD_PROMPT);
    if will_save_world {
        Player::new(name.clone(), class)
    } else {
        panic!("The world was not saved.");
    }
}

pub fn select_from_save_file_menu() -> SaveFileSelection {
    let existing_saves = save::find_existing_saves();
    println!("Existing saves found:");
    let saves = existing_saves
        .iter()
        .fold(String::new(), |mut acc: String, next| -> String {
            acc.push_str(next);
            acc.push('\n');
            acc
        });
    print!("{saves}");
    // TODO: create prompt here to get selection
}

pub fn start() -> Player {
    let save_file_selection = select_from_save_file_menu();
    match save_file_selection {
        SaveFileSelection::ExistingSave(name) => save::load_save_file(&name).unwrap(),
        SaveFileSelection::NewSave => create_new_save(),
    }
}
