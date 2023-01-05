pub mod class;
pub mod start_game;
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
    options: Vec<PromptOption>,
}

impl InputPrompt {
    fn generate_prompt_with_options(&self) -> String {
        let mut prompt = String::from(self.initial_prompt);
        self.options.iter().for_each(|option| {
            let option_str = format!(
                "({short_name}) {name}",
                short_name = option.short_name,
                name = option.name,
            );
            if !prompt.is_empty() {
                prompt.push('\n');
            }
            prompt.push_str(&option_str);
        });
        prompt
    }

    fn generate_reprompt_with_options(&self) -> String {
        format!(
            "Please enter a valid option. Valid options are: {}",
            self.options.iter().fold(String::new(), |mut acc: String, option: &PromptOption| -> String {
                if !acc.is_empty() {
                    acc.push_str(", ");
                }
                acc.push_str(option.short_name);
                acc
            }),
        )
    }

    pub fn show(self) -> PromptOption {
        let mut selected_option: Option<PromptOption> = None;
        let prompt = self.generate_prompt_with_options();
        let mut answer = io_util::request_input(prompt).to_lowercase();
        loop {
            self.options.iter().for_each(|option| -> () { 
                if answer == option.short_name.to_lowercase() || answer == option.name.to_lowercase() {
                    selected_option = Some(option.clone());
                }
            });

            if selected_option.is_some() {
                break;
            } else {
                let reprompt = self.generate_reprompt_with_options();
                answer = io_util::request_input(reprompt).to_lowercase();
            }
        }
        selected_option.expect("Failed to retrieve selected option from prompt")
    }
}
