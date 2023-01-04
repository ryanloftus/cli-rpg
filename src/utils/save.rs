use std::fs::File;
use crate::player::Player;

fn player_name_to_file_name(player_name: &String) -> String {
    format!("{}_save_data.txt", player_name)
}

// Creates a save file for the player and returns the name of the file
pub fn create_save_file(player: &Player) {
    let file_name = player_name_to_file_name(&player.name);
    let create_file_result = File::create(file_name.clone());
    if create_file_result.is_err() {
        let error_message = create_file_result.expect_err("").to_string();
        panic!("Failed to create save data file with name {file_name}. Error: {error_message}");
    }
    // TODO: call save to fill in the file's contents
}

pub fn _save(_player: &Player) {
    todo!("write the player to the save file")
}
