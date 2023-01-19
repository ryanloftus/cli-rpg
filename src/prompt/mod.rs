pub mod io_util;

use std::option::Option;

const REPROMPT_OPTION: &str = "reprompt";

pub trait PromptOption {
    fn option_name(&self) -> String;
    fn short_option_name(&self) -> Option<String>;
}

#[derive(Debug, Clone)]
pub struct InputPrompt<T: PromptOption> {
    initial_prompt: String,
    options: Vec<T>,
}

impl<T: PromptOption> InputPrompt<T> {
    pub fn show_and_get_selection(self) -> T {
        let prompt = self.generate_prompt_with_options();
        let reprompt = self.generate_reprompt_with_options();
        let mut answer = io_util::request_input(&prompt).to_lowercase();
        loop {
            if answer == REPROMPT_OPTION {
                answer = io_util::request_input(&prompt);
                continue;
            }

            let selection = self
                .options
                .iter()
                .find(|option| Self::is_option_selected(**option, answer));
            if let Some(selection) = selection {
                return *selection;
            }

            answer = io_util::request_input(&reprompt).to_lowercase();
        }
    }

    fn is_option_selected(option: T, selection: String) -> bool {
        let lowercase_selection = selection.to_lowercase();
        if let Some(short_option_name) = option.short_option_name() {
            if short_option_name.to_lowercase() == lowercase_selection {
                return true;
            }
        }
        return option.option_name().to_lowercase() == lowercase_selection;
    }

    fn generate_prompt_with_options(&self) -> String {
        let mut prompt = self.initial_prompt.to_string();
        self.options.iter().for_each(|option| {
            prompt.push_str(&Self::get_prompt_option_string(*option));
        });
        return prompt;
    }

    fn generate_reprompt_with_options(&self) -> String {
        format!(
            "Please enter a valid option. Enter \"{reprompt_option}\" to show options again.",
            reprompt_option = REPROMPT_OPTION,
        )
    }

    fn get_prompt_option_string(option: T) -> String {
        let option_name = option.option_name();
        if let Some(short_name) = option.short_option_name() {
            format!(
                "({short_name}) {name}\n",
                short_name = short_name,
                name = option_name,
            )
        } else {
            option_name
        }
    }
}
