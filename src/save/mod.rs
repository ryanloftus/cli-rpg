use crate::player::Player;
use serde_json;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Write;

const SAVE_FILE_SUFFIX: &str = "_save_data.json";

fn get_current_working_dir() -> String {
    std::env::current_dir()
        .expect("Failed to get current working directory")
        .into_os_string()
        .into_string()
        .unwrap()
}

fn player_name_to_file_path(player_name: &String) -> String {
    format!(
        "{dir}\\{player_name}{suffix}",
        dir = get_current_working_dir(),
        player_name = player_name,
        suffix = SAVE_FILE_SUFFIX,
    )
}

fn player_to_json(player: &Player) -> String {
    serde_json::to_string_pretty(player).expect("Failed to serialize player.")
}

fn open_save_file(player_name: &String) -> File {
    let file_name = player_name_to_file_path(&player_name);
    std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(file_name)
        .unwrap()
}

fn open_save_file_readonly(player_name: &String) -> File {
    let file_name = player_name_to_file_path(&player_name);
    File::open(file_name).unwrap()
}

pub fn find_existing_saves() -> Vec<String> {
    let dir = get_current_working_dir();
    let dir_len = dir.len();
    let files_in_dir = std::fs::read_dir(dir).unwrap();
    let existing_save_files = files_in_dir
        .filter(|file| file.is_ok())
        .map(|file| file.unwrap().path().display().to_string())
        .filter(|path| path.ends_with(SAVE_FILE_SUFFIX))
        .map(|path| -> String {
            let name_len = path.len() - dir_len - SAVE_FILE_SUFFIX.len() - 1;
            let name_start = dir_len + 1;
            let name_end = name_start + name_len;
            path[name_start..name_end].to_string()
        });
    existing_save_files.collect()
}

pub fn save(player: &Player) {
    let file = open_save_file(&player.name);
    let contents = player_to_json(player);
    let mut writer = BufWriter::new(file);
    writer
        .write_all(contents.as_bytes())
        .expect("Write to save file failed");
}

pub fn load_save_file(player_name: &String) -> Result<Player, Box<dyn Error>> {
    let file = open_save_file_readonly(player_name);
    let reader = BufReader::new(file);
    let player = serde_json::from_reader(reader)?;
    Ok(player)
}
