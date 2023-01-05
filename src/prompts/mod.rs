pub mod class_prompt;
pub mod new_player_prompt;
pub mod after_battle;

use crate::utils::io_util;
use std::option::Option;

#[derive(Debug, Clone)]
pub struct PromptOption {
    name: &'static str,
    short_name: &'static str,
}

#[derive(Debug, Clone)]
pub struct InputPrompt {
    initial_prompt: &'static str,
    options: &'static [PromptOption],
}

impl InputPrompt {
    pub fn show(self) -> PromptOption {
        let mut selected_option: Option<PromptOption> = None;
        let mut answer = io_util::request_input(self.initial_prompt);
        loop {
            self.options.iter().for_each(|option| -> () { 
                if answer == option.short_name || answer == option.name {
                    selected_option = Some(option.clone());
                }
            });

            if selected_option.is_some() {
                break;
            } else {
                answer = io_util::request_input("Please enter a valid option.");
            }
        }
        selected_option.expect("Failed to retrieve selected option from prompt")
    }
}
