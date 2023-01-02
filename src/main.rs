use std::io::stdin;

mod save;

fn my_read_line() -> String {
    let mut input_buffer = String::new();
    let result = stdin().read_line(&mut input_buffer);
    if result.is_err() {
        panic!("Failed to read from console, ending game 😕");
    }
    // TODO: can this be cleaned up? surely we don't actually have to clone this twice...
    let name = input_buffer.clone();
    name.trim_end().to_owned()
}

fn main() {
    // TODO make sure name only contains alphanumeric or _ to make a valid save data file name.
    println!("What is your name?");
    let name = my_read_line();
    println!("Hello, {name}!");
    save::save::create_save_file(name);
}
