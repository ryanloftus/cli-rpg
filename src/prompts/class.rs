use crate::class;
use super::InputPrompt;
use super::PromptOption;

// TODO: make this function generic and take a list of classes as a parameter
pub fn starting_class_prompt() -> class::Class {
    let prompt = InputPrompt {
        initial_prompt: "Choose a class.",
        options: class::STARTER_CLASSES.map(|class| -> PromptOption {
            PromptOption {
                name: class.name,
                short_name: class.unique_id,
            }
        }).to_vec(),
    };
    let class = prompt.show();
    class::STARTER_CLASSES
        .iter()
        .find(|starter_class| starter_class.unique_id == class.short_name)
        .expect("Failed to get class from input")
        .clone()
}
