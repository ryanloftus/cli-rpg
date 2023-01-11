use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryProgress {
    pub areas_completed: u8,
    pub current_area_progress: u8,
}
