mod save;
mod io_util;

fn is_valid_name(name: String) -> bool {
    name.chars().all(char::is_alphabetic)
}

fn main() {
    let name = io_util::request_input(
        "What is your name?",
        is_valid_name,
        "Name must be alphetic (letters only). Please enter a different name."
    );
    println!("Hello, {name}!");
    save::create_save_file(name);
}
