use super::{PromptOption, ToPromptOption};
use crate::class::prompt::starting_class_prompt;
use crate::player::Player;
use crate::prompt::{InputPrompt, io_util};
use crate::save;

const WILL_SAVE_WORLD_PROMPT: &str = "Now that you are able to fight... The world is in need of a hero...
Territories far and wide have all been suffering from wars, monster attacks, and natural disasters.
Will you help save the world? [Y/N]";

const NEW_SAVE_OPTION: PromptOption = PromptOption {
    name: std::borrow::Cow::Borrowed("new"),
    short_name: Some("n"),
};

enum SaveFileSelection {
    ExistingSave(String),
    NewSave,
}

impl ToPromptOption for SaveFileSelection {
    fn to_prompt_option(&self) -> PromptOption {
        
    }
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

fn select_from_save_file_menu() -> SaveFileSelection {
    let existing_saves = save::find_existing_saves();
    let mut options: Vec<PromptOption> = existing_saves.iter().map(|name| PromptOption {
        name: std::borrow::Cow::Owned(name.clone()),
        short_name: None,
    }).collect();
    options.push(NEW_SAVE_OPTION);
    let initial_prompt = format!(
        "Select a save to load or enter \"{new_save_option}\"",
        new_save_option = NEW_SAVE_OPTION.name,
    );
    let prompt = InputPrompt {
        initial_prompt,
        options,
    };
    let answer = prompt.show();
    if answer.name == NEW_SAVE_OPTION.name {
        SaveFileSelection::NewSave
    } else {
        SaveFileSelection::ExistingSave(answer.name.to_string())
    }
}

pub fn start() -> Player {
    let save_file_selection = select_from_save_file_menu();
    match save_file_selection {
        SaveFileSelection::ExistingSave(name) => save::load_save_file(&name).unwrap(),
        SaveFileSelection::NewSave => create_new_save(),
    }
}
