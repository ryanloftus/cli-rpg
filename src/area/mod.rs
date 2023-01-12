mod plains;
mod forest;
mod mountains;
mod islands;
mod story;

use crate::battle::do_battle;
use crate::player::Player;
use crate::prompts::before_battle::{self, BeforeBattleAction};
use story::StoryComponent;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
}

impl Area {
    /*
     * Starts or continues players progress in an area from the given entry point
     * entry_point is an idx in area.story
     */
    pub fn enter(&self, mut player: &Player, entry_point: u8) {
        for i in usize::from(entry_point)..self.story.len() {
            match self.story[i] {
                StoryComponent::Text(text) => println!("{text}"),
                StoryComponent::Enemy(enemy) => {
                    let action = before_battle::show_enemy_prompt();
                    match action {
                        BeforeBattleAction::Fight(num_enemies) => do_battle(player, enemy),
                        BeforeBattleAction::ReturnToPreviousArea => todo!("prompt which area"),
                    }
                },
                StoryComponent::Boss(enemy) => {
                    let action = before_battle::show_boss_prompt();
                    match action {
                        BeforeBattleAction::Fight(num_enemies) => do_battle(player, enemy),
                        BeforeBattleAction::ReturnToPreviousArea => do_battle(player, enemy),
                    }
                },
            }
        }
    }
    
    /*
     * Should only be called once the player has completed this areas story
     * Player returns to the area to train by fighting practice battles
     */
    pub fn train(&self, mut player: &Player) {
        todo!("implement training");
    }
}

pub fn build_areas() -> [Area; 1] {
    [plains::new()] // TODO: add more areas
}
