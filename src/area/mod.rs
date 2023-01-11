mod plains;
mod forest;
mod mountains;
mod islands;
mod story;

use crate::player::Player;
use crate::battle::do_battle;
use story::StoryComponent;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
}

impl Area {
    /*
     * Starts or continues players progress in an area from the given entry point (which is an idx in the story)
     */
    pub fn enter(&self, mut player: &Player, entry_point: u8) {
        for i in usize::from(entry_point)..self.story.len() {
            match &self.story[i] {
                StoryComponent::Text(text) => println!("{text}"),
                StoryComponent::Enemy(enemy) => do_battle(player, enemy),
                StoryComponent::Boss(enemy) => do_battle(player, enemy),
            }
        }
    }
    
    // TODO: add way to return to prev area to train
}

pub fn build_areas() -> [Area; 1] {
    [plains::new()] // TODO: add more areas
}
