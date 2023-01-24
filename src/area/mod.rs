mod caves;
mod forest;
mod icefields;
mod islands;
mod kingdom;
mod mountains;
mod plains;
mod story;

use crate::player::Player;
use crate::save;
use crate::{battle::battle, prompt::PromptOption};
use story::StoryComponent;

use self::story::{story_component_prompt, StoryComponentAction};

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

impl Area {
    /*
     * Starts or continues players progress in an area from the given entry point
     * entry_point is an idx in area.story
     */
    pub fn enter(&self, player: &mut Player) -> AreaResult {
        // TODO: it might make sense to shift more of the weight of this fn to game. This could simplify control flow,
        //   and limit coupling (ie only call save from game).
        while player.story_progress.current_area_progress < self.story.len() {
            let action = self.get_action(player.story_progress.current_area_progress);
            match action {
                StoryComponentAction::ShowText(text) => {
                    println!("{text}");
                    player.story_progress.current_area_progress += 1;
                }
                StoryComponentAction::Battle(enemies) => {
                    if !battle(&player, &enemies) {
                        return AreaResult::PlayerWasDefeated;
                    } else {
                        player.experience.enemies_defeated(&enemies);
                    }
                    player.story_progress.current_area_progress += enemies.len();
                }
                StoryComponentAction::BossBattle(boss) => {
                    let enemies = vec![boss];
                    if !battle(&player, &enemies) {
                        return AreaResult::PlayerWasDefeated;
                    } else {
                        player.experience.enemies_defeated(&enemies);
                    }
                    player.story_progress.current_area_progress += 1;
                }
                StoryComponentAction::ReturnToPreviousArea => {
                    return AreaResult::ReturnToPreviousArea
                }
            }
            save::save(&player);
        }
        return AreaResult::AreaCompleted;
    }

    /*
     * Should only be called once the player has completed this areas story
     * Player returns to the area to train by fighting practice battles
     */
    pub fn train(&self, player: &mut Player) {
        todo!("implement training");
    }

    fn get_action(&self, story_idx: usize) -> StoryComponentAction {
        match &self.story[story_idx] {
            StoryComponent::Text(text) => StoryComponentAction::ShowText(text.clone()),
            StoryComponent::Enemy(_) => {
                let mut enemies = Vec::new();
                for j in story_idx..self.story.len() {
                    if let StoryComponent::Enemy(enemy) = &self.story[j] {
                        enemies.push(enemy);
                    }
                }
                story_component_prompt::show_enemy_prompt(enemies)
            }
            StoryComponent::Boss(boss) => story_component_prompt::show_boss_prompt(&boss),
        }
    }
}

pub fn build_areas() -> [Area; 7] {
    return [kingdom::new(), plains::new(), forest::new(), islands::new(), caves::new(), icefields::new(), mountains::new()];
}
