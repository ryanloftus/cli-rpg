use crate::class;
use crate::player::Player;
use crate::prompt::{PromptOption, io_util, get_selection_from_options};
use crate::save;

const WILL_SAVE_WORLD_PROMPT: &str =
    "Now that you are able to fight... The world is in need of a hero...
Territories far and wide have all been suffering from wars, monster attacks, and natural disasters.
Will you help save the world? [Y/N]";

const NEW_SAVE_OPTION: &str = "New";
const NEW_SAVE_SHORT_OPTION: &str = "N";

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
    let save_file_selection = select_from_save_file_menu();
    match save_file_selection {
        SaveFileSelection::ExistingSave(name) => save::load_save_file(&name).unwrap(),
        SaveFileSelection::NewSave => create_new_save(),
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
    return name.chars().all(char::is_alphabetic) &&
        lowercase_name != NEW_SAVE_OPTION.to_lowercase() &&
        lowercase_name != NEW_SAVE_SHORT_OPTION.to_lowercase() &&
        existing_names
            .iter()
            .map(|existing_name| existing_name.to_lowercase())
            .find(|lowercase_existing_name| *lowercase_existing_name == lowercase_name).is_none();
}

fn create_new_save() -> Player {
    let existing_names = save::find_existing_saves();
    let name = io_util::request_input_with_validation(
        "What is your name?",
        |name| is_valid_name(name, &existing_names),
        "Name must satisfy the following conditions:
        1. it has not already been used by an existing save
        2. it is not \"N\" or \"New\" (case-insensitive)
        3. it is made up of only alphabetic characters
        Please enter a different name.",
    );
    println!("Hello, {name}!");
    let class = class::choose_class_prompt(&class::STARTER_CLASSES);
    let will_save_world = io_util::request_yes_or_no(WILL_SAVE_WORLD_PROMPT);
    if will_save_world {
        Player::new(name.clone(), class)
    } else {
        panic!("The world was not saved.");
    }
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
    return get_selection_from_options(prompt, &options);
}
