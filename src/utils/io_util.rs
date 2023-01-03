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
