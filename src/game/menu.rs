use crate::prompt::{self, PromptOption};
use crate::save;
use crate::unit::player::Player;

const NEW_SAVE_OPTION: &str = "New";
const NEW_SAVE_SHORT_OPTION: &str = "N";
const INVALID_NAME_MESSAGE: &str = "Name must satisfy the following conditions:
1. it has not already been used by an existing save
2. it is not \"N\" or \"New\" (case-insensitive)
3. it is made up of only alphabetic characters
Please enter a different name.";

#[derive(Clone)]
enum SaveFileSelection {
    ExistingSave(String),
    NewSave,
}

impl PromptOption for SaveFileSelection {
    fn option_name(&self) -> String {
        match self {
            SaveFileSelection::NewSave => String::from(NEW_SAVE_OPTION),
            SaveFileSelection::ExistingSave(save_name) => String::from(save_name),
        }
    }

    fn short_option_name(&self) -> Option<String> {
        match self {
            SaveFileSelection::NewSave => Some(String::from(NEW_SAVE_SHORT_OPTION)),
            SaveFileSelection::ExistingSave(_) => None,
        }
    }
}

pub fn start() -> Player {
    loop {
        let save_file_selection = select_from_save_file_menu();
        match save_file_selection {
            SaveFileSelection::ExistingSave(name) => {
                let player = save::load_save_file(&name);
                if player.is_err() {
                    let err = player.unwrap_err();
                    println!("{err}");
                } else {
                    return player.unwrap();
                }
            }
            SaveFileSelection::NewSave => return create_new_save(),
        }
    }
}

/*
 * Returns true if the player name (which is also the save file name) is valid
 * the player name is valid if:
 * 1. it has not already been used by a previous save,
 * 2. it is not equal to the new save option,
 * 3. it is alphabetic characters only
 */
fn is_valid_name(name: String, existing_names: &Vec<String>) -> bool {
    let lowercase_name = name.to_lowercase();
    return name.chars().all(char::is_alphabetic)
        && lowercase_name != NEW_SAVE_OPTION.to_lowercase()
        && lowercase_name != NEW_SAVE_SHORT_OPTION.to_lowercase()
        && existing_names
            .iter()
            .map(|existing_name| existing_name.to_lowercase())
            .find(|lowercase_existing_name| *lowercase_existing_name == lowercase_name)
            .is_none();
}

fn create_new_save() -> Player {
    let existing_names = save::find_existing_saves();
    let name = prompt::get_selection_from_custom_validation(
        "What is your name?",
        |name| is_valid_name(name, &existing_names),
        INVALID_NAME_MESSAGE,
    );
    println!("Hello, {name}!");
    return Player::new(name.clone());
}

fn select_from_save_file_menu() -> SaveFileSelection {
    let existing_saves = save::find_existing_saves();
    let mut options: Vec<SaveFileSelection> = existing_saves
        .iter()
        .map(|name| SaveFileSelection::ExistingSave(name.clone()))
        .collect();
    options.push(SaveFileSelection::NewSave);
    let prompt = format!(
        "Select a save to load or enter \"{new_save_option}\"",
        new_save_option = NEW_SAVE_OPTION,
    );
    return prompt::get_selection_from_options(prompt, &options);
}
