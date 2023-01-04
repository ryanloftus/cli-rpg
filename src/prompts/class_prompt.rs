use crate::class;
use crate::utils::io_util;

// a class name is valid only if it is equal to a class' unique_id or name
fn is_valid_class(class: String, options: &[class::Class]) -> bool {
    if class.len() == 1 {
        options
            .iter()
            .any(|real_class| class == real_class.unique_id)
    } else {
        options.iter().any(|real_class| class == real_class.name)
    }
}

fn is_valid_starter_class(class: String) -> bool {
    is_valid_class(class, &class::STARTER_CLASSES)
}

fn starter_class_from_string(class: String) -> class::Class {
    if class.len() == 1 {
        class::STARTER_CLASSES
            .iter()
            .find(|real_class| class == real_class.unique_id)
            .expect("")
            .clone()
    } else {
        class::STARTER_CLASSES
            .iter()
            .find(|real_class| class == real_class.name)
            .expect("")
            .clone()
    }
}

pub fn starting_class_prompt() -> class::Class {
    println!("What class of fighter are you?");
    class::STARTER_CLASSES
        .iter()
        .for_each(|class| println!("{}", class.to_option_string()));
    let class = io_util::request_input(
        "Choose a class.",
        is_valid_starter_class,
        "Please enter a valid class option. Valid options are \"S\", \"K\", \"B\", \"M\", \"H\", and \"G\"",
    );
    starter_class_from_string(class)
}
