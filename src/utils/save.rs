use std::fs::File;
use std::io::BufWriter;
use std::io::Write;
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

// TODO: save skills
fn get_save_file_contents(player: &Player) -> String {
    format!(
"{{
    name: {player_name},
    class: {class_id},
    experience: {{
        level: {level},
        experience_towards_next_level: {experience_towards_next_level},
    }},
    skills: [
        // TODO
    ],
    story_progress: {{
        areas_completed: {areas_completed},
        enemies_defeated_in_current_area: {enemies_defeated_in_current_area},
    }}
}}",
        player_name = player.name,
        class_id = player.class.unique_id,
        level = player.experience.level,
        experience_towards_next_level = player.experience.experience_towards_next_level,
        areas_completed = player.story_progress.areas_completed,
        enemies_defeated_in_current_area = player.story_progress.enemies_defeated_in_current_area,
    )
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
