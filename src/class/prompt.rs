use crate::class;
use crate::prompt::InputPrompt;

// TODO: make this function generic and take a list of classes as a parameter
pub fn starting_class_prompt() -> class::Class {
    let prompt = InputPrompt {
        initial_prompt: String::from("Choose a class."),
        options: class::STARTER_CLASSES.to_vec(),
    };
    let class = prompt.show_and_get_selection();
    return class;
}
