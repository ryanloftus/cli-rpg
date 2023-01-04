use std::io::stdin;

pub fn my_read_line() -> String {
    let mut input_buffer = String::new();
    let result = stdin().read_line(&mut input_buffer);
    if result.is_err() {
        panic!("Failed to read from console, ending game 😕");
    }
    input_buffer.clone().trim_end().to_owned()
}

pub fn request_input(
    prompt: &str,
    validation_predicate: fn(String) -> bool,
    reprompt: &str,
) -> String {
    println!("{prompt}");
    let mut input = my_read_line();
    while !validation_predicate(input.clone()) {
        println!("{reprompt}");
        input = my_read_line();
    }
    input
}

fn is_yes(answer: &String) -> bool {
    answer.to_ascii_lowercase().starts_with("y")
}

fn is_no(answer: &String) -> bool {
    answer.to_ascii_lowercase().starts_with("n")
}

fn is_valid_yes_or_no(answer: String) -> bool {
    is_yes(&answer) || is_no(&answer)
}

pub fn request_yes_or_no(prompt: &str) -> bool {
    let answer = request_input(
        prompt,
        is_valid_yes_or_no,
        "Please enter yes (Y) or no (N).",
    );
    is_yes(&answer)
}
