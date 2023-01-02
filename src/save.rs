pub mod save {
    use std::fs::File;
    
    fn player_name_to_file_name(player_name: String) -> String {
        format!("{}_save_data.txt", player_name).to_ascii_lowercase()
    }


    /// Creates a save file for the player and returns the name of the file
    pub fn create_save_file(player_name: String) -> String {
        let file_name = player_name_to_file_name(player_name);
        let create_file_result = File::create(file_name.clone());
        if create_file_result.is_err() {
            let error_message = create_file_result.expect_err("").to_string();
            panic!("Failed to create save data file with name {file_name}. Error: {error_message}");
        }
        file_name
    }

    pub fn update_save_file(player_name: String) {
        todo!("add SaveData struct that can be passed as a param here to update the save file with")
    }
}
