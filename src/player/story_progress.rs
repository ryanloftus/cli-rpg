use serde::{ Deserialize, Serialize };

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryProgress {
    pub areas_completed: usize,
    pub current_area_progress: usize,
}
