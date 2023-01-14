mod plains;
mod forest;
mod mountains;
mod islands;
mod story;

use crate::battle::do_battle;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::prompts::story_component::{self, StoryComponentAction};
use story::StoryComponent;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
}

/*
 * Why is the player leaving the area
 */
enum AreaResult {
    ReturnToPreviousArea,
    AreaCompleted,
    PlayerWasDefeated,
}

impl Area {
    /*
     * Starts or continues players progress in an area from the given entry point
     * entry_point is an idx in area.story
     */
    pub fn enter(&self, mut player: &Player, entry_point: u8) {
        // TODO: add auto save after each component is completed
        // TODO: return an area result
        // TODO: add a wrapper object "Game" called by main that handles between area logic
        for i in usize::from(entry_point)..self.story.len() {
            let action = self.get_action(i);
            match action {
                StoryComponentAction::ShowText(text) => println!("{text}"),
                StoryComponentAction::Battle(num_enemies) => {
                    // TODO: pass enemy vector into num_enemies prompt and preview the upcoming enemies within the prompt,
                    // then include the enemy vector in StoryComponentAction::Battle instead of num_enemies
                    let enemies = self
                        .story[i..(i + usize::from(num_enemies))]
                        .iter()
                        .map(|story_component| -> Enemy {
                            if let StoryComponent::Enemy(enemy) = story_component {
                                enemy.clone()
                            } else {
                                panic!("")
                            }
                        })
                        .collect();
                    do_battle(player, enemies);
                },
                StoryComponentAction::BossBattle(boss) => do_battle(player, vec![boss]),
                StoryComponentAction::ReturnToPreviousArea => break,
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

    fn get_action(&self, story_idx: usize) -> StoryComponentAction {
        match &self.story[story_idx] {
            StoryComponent::Text(text) => StoryComponentAction::ShowText(text.clone()),
            StoryComponent::Enemy(_) => {
                let mut max_enemies = 0;
                for j in story_idx..self.story.len() {
                    if let StoryComponent::Enemy(_) = self.story[j] {
                        max_enemies += 1;
                    }
                }
                story_component::show_enemy_prompt(max_enemies)
            },
            StoryComponent::Boss(boss) => story_component::show_boss_prompt(&boss)
        }
    }
}

pub fn build_areas() -> [Area; 1] {
    [plains::new()] // TODO: add more areas
}
