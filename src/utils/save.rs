use std::error::Error;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
use std::io::BufReader;
use std::path::Path;
use serde_json;
use crate::player::Player;

fn get_current_working_dir() -> String {
    std::env::current_dir()
        .expect("Failed to get current working directory")
        .into_os_string()
        .into_string()
        .unwrap()
}

fn player_name_to_file_path(player_name: &String) -> String {
    format!(
        "{dir}\\{player_name}_save_data.txt",
        dir = get_current_working_dir(),
        player_name = player_name,
    )
}

fn get_save_file_contents(player: &Player) -> String {
    serde_json::to_string_pretty(player).expect("Failed to serialize player.")
}

fn open_save_file(player_name: &String) -> File {
    let file_name = player_name_to_file_path(&player_name);
    File::create(file_name.clone()).expect("Could not create/open the save file.")
}

pub fn save(player: &Player) {
    let file = open_save_file(&player.name);
    let contents = get_save_file_contents(player);
    let mut writer = BufWriter::new(file);
    writer.write_all(contents.as_bytes()).expect("Write to save file failed");
}

pub fn has_save_file(player_name: &String) -> bool {
    let save_file_path = player_name_to_file_path(player_name);
    let save_file_path = Path::new(&save_file_path);
    save_file_path.exists()
}

pub fn load_save_file(player_name: &String) -> Result<Player, Box<dyn Error>> {
    let file = open_save_file(player_name);
    let reader = BufReader::new(file);
    let mut player = serde_json::from_reader(reader)?;
    Ok(player)
}
