pub mod io_util;

use std::option::Option;

const REPROMPT_OPTION: &str = "reprompt";

pub trait PromptOption {
    fn option_name(&self) -> String;
    fn short_option_name(&self) -> Option<String>;
}

// TODO: expand this module to include functions for prompting for numbers, boolean (y/n), or custom (ie entering player name)
pub fn get_selection_from_options<T: PromptOption + Clone>(prompt: String, options: &Vec<T>) -> T {
    // TODO: fix trailing newline issue (best solution might be to keep it but use print! instead of println! in io_util)
    let prompt = generate_prompt_with_options(prompt, options);
    let reprompt = generate_reprompt();
    let mut answer = io_util::request_input(&prompt).to_lowercase();
    loop {
        if answer == REPROMPT_OPTION {
            answer = io_util::request_input(&prompt);
            continue;
        }

        let selection = options.iter()
            .find(|option| is_option_selected((*option).clone(), answer.clone()));
        if let Some(selection) = selection {
            return selection.clone();
        }

        answer = io_util::request_input(&reprompt).to_lowercase();
    }
}

fn is_option_selected<T : PromptOption>(option: T, selection: String) -> bool {
    let lowercase_selection = selection.to_lowercase();
    if let Some(short_option_name) = option.short_option_name() {
        if short_option_name.to_lowercase() == lowercase_selection {
            return true;
        }
    }
    return option.option_name().to_lowercase() == lowercase_selection;
}

fn generate_prompt_with_options<T : PromptOption>(prompt: String, options: &Vec<T>) -> String {
    let mut prompt_with_options = prompt.to_string();
    prompt_with_options.push('\n');
    options.iter().for_each(|option| {
        prompt_with_options.push_str(&get_prompt_option_string(option));
    });
    return prompt_with_options;
}

fn generate_reprompt() -> String {
    format!(
        "Please enter a valid option. Enter \"{reprompt_option}\" to show options again.",
        reprompt_option = REPROMPT_OPTION,
    )
}

fn get_prompt_option_string<T : PromptOption>(option: &T) -> String {
    let option_name = option.option_name();
    if let Some(short_name) = option.short_option_name() {
        format!(
            "({short_name}) {name}\n",
            short_name = short_name,
            name = option_name,
        )
    } else {
        format!("{option}\n", option = option_name)
    }
}
