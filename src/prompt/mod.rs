mod io_util;
pub mod start_game;
pub mod story_component;

use std::option::Option;

const REPROMPT_OPTION: &str = "reprompt";

pub trait ToPromptOption {
    fn to_prompt_option(&self) -> PromptOption;
}

// TODO: Convert InputPrompt methods to use generics
#[derive(Debug, Clone)]
pub struct PromptOption {
    name: String,
    short_name: Option<String>,
}

impl PromptOption {
    pub fn to_string(&self) -> String {
        if let Some(short_name) = self.short_name {
            format!(
                "({short_name}) {name}",
                short_name = short_name,
                name = self.name,
            )
        } else {
            self.name.to_string()
        }
    }
}

#[derive(Debug, Clone)]
pub struct InputPrompt<T : ToPromptOption> {
    initial_prompt: String,
    options: Vec<T>,
}

impl<T : ToPromptOption> InputPrompt<T> {
    fn generate_prompt_with_options(&self) -> String {
        let mut prompt = self.initial_prompt.to_string();
        self.options.iter().for_each(|option| {
            if !prompt.is_empty() {
                prompt.push('\n');
            }
            prompt.push_str(&option.to_string());
        });
        prompt
    }

    fn generate_reprompt_with_options(&self) -> String {
        format!(
            "Please enter a valid option. Enter \"{reprompt_option}\" to show options again.",
            reprompt_option = REPROMPT_OPTION,
        )
    }

    pub fn show(self) -> PromptOption {
        let mut selected_option: Option<PromptOption> = None;
        let prompt = self.generate_prompt_with_options();
        let mut answer = io_util::request_input(&prompt).to_lowercase();
        loop {
            if answer == REPROMPT_OPTION {
                answer = io_util::request_input(&prompt);
                continue;
            }

            self.options.iter().for_each(|option| -> () {
                if (option.short_name.is_some()
                    && answer == option.short_name.unwrap().to_lowercase())
                    || answer == option.name.to_lowercase()
                {
                    selected_option = Some(option.clone());
                }
            });

            if selected_option.is_some() {
                break;
            } else {
                let reprompt = self.generate_reprompt_with_options();
                answer = io_util::request_input(&reprompt).to_lowercase();
            }
        }
        selected_option.expect("Failed to retrieve selected option from prompt")
    }
}
