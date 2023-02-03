mod caves;
mod forest;
mod icefields;
mod islands;
mod kingdom;
mod mountains;
mod plains;

use crate::enemy::Enemy;
use crate::prompt::PromptOption;

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

impl PartialEq for Area {
    fn eq(&self, other: &Self) -> bool {
        return self.short_option_name() == other.short_option_name();
    }
}

impl Eq for Area {}

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Text(String),
    Enemy(Enemy),
    Boss(Enemy),
    // TODO: GainAttribute once player attributes are added (i.e. DemonSlayer after progressing forest)
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
