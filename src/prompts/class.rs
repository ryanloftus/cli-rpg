use crate::class;
use super::InputPrompt;
use super::PromptOption;
use std::borrow::Cow;

// TODO: make this function generic and take a list of classes as a parameter
pub fn starting_class_prompt() -> class::Class {
    let prompt = InputPrompt {
        initial_prompt: "Choose a class.",
        options: class::STARTER_CLASSES.map(|class| -> PromptOption {
            PromptOption {
                name: Cow::Borrowed(class.name),
                short_name: Some(class.unique_id),
            }
        }).to_vec(),
    };
    let class = prompt.show();
    class::STARTER_CLASSES
        .iter()
        .find(|starter_class| class.short_name.is_some() && starter_class.unique_id == class.short_name.unwrap())
        .expect("Failed to get class from input")
        .clone()
}
