mod io_util;

use std::option::Option;

const REPROMPT_OPTION: &str = "reprompt";

/// Prints the prompt and reads in a number from the user
pub fn get_selection_from_numeric_range(prompt: &str, min: i32, max: i32) -> i32 {
    return io_util::request_num(prompt, min, max);
}

/// Prints the prompt and reads in a yes or no from the user, yes equates to true, and no equates to false
pub fn _get_selection_from_boolean(prompt: &str) -> bool {
    return io_util::_request_yes_or_no(prompt);
}

/// Prints the prompt and reads in a string that meets the validation requirements
pub fn get_selection_from_custom_validation(
    prompt: &str,
    validation_predicate: impl Fn(String) -> bool,
    reprompt: &str,
) -> String {
    return io_util::request_input_with_validation(prompt, validation_predicate, reprompt);
}

pub trait PromptOption {
    fn option_name(&self) -> String;
    fn short_option_name(&self) -> Option<String>;
}

/// Prints the prompt and options and reads in an option from the user
pub fn get_selection_from_options<T: PromptOption + Clone>(prompt: String, options: &Vec<T>) -> T {
    let prompt = generate_prompt_with_options(prompt, options);
    let reprompt = generate_reprompt();
    let mut answer = io_util::request_input(&prompt).to_lowercase();
    loop {
        if answer == REPROMPT_OPTION {
            answer = io_util::request_input(&prompt);
            continue;
        }

        let selection = options
            .iter()
            .find(|option| is_option_selected((*option).clone(), answer.clone()));
        if let Some(selection) = selection {
            return selection.clone();
        }

        answer = io_util::request_input(&reprompt).to_lowercase();
    }
}

fn is_option_selected<T: PromptOption>(option: T, selection: String) -> bool {
    let lowercase_selection = selection.to_lowercase();
    if let Some(short_option_name) = option.short_option_name() {
        if short_option_name.to_lowercase() == lowercase_selection {
            return true;
        }
    }
    return option.option_name().to_lowercase() == lowercase_selection;
}

fn generate_prompt_with_options<T: PromptOption>(prompt: String, options: &Vec<T>) -> String {
    let mut prompt_with_options = prompt.to_string();
    prompt_with_options.push('\n');
    options.iter().for_each(|option| {
        prompt_with_options.push_str(&get_prompt_option_string(option));
    });
    return prompt_with_options;
}

fn generate_reprompt() -> String {
    format!(
        "Please enter a valid option. Enter \"{reprompt_option}\" to show options again.\n",
        reprompt_option = REPROMPT_OPTION,
    )
}

fn get_prompt_option_string<T: PromptOption>(option: &T) -> String {
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
