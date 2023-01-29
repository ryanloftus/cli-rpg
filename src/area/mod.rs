mod caves;
mod forest;
mod icefields;
mod islands;
mod kingdom;
mod mountains;
mod plains;
mod story;

use crate::prompt::PromptOption;
use story::StoryComponent;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
}

impl PromptOption for Area {
    fn option_name(&self) -> String {
        String::from(self.name)
    }

    fn short_option_name(&self) -> Option<String> {
        Some(String::from(self.unique_id))
    }
}

/*
 * Why is the player leaving the area
 */
pub enum AreaResult {
    ReturnToPreviousArea,
    AreaCompleted,
    PlayerWasDefeated,
}

pub const NUM_AREAS: usize = 7;

pub fn build_areas() -> Vec<Area> {
    return vec![
        kingdom::new(),
        plains::new(),
        forest::new(),
        islands::new(),
        caves::new(),
        icefields::new(),
        mountains::new(),
    ];
}
