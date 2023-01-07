use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryProgress {
    pub areas_completed: u8,
    pub enemies_defeated_in_current_area: u8,
}
