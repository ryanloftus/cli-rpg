mod forest;
mod islands;
mod mountains;
mod plains;
mod story;

use crate::battle::battle;
use crate::player::Player;
use crate::save;
use story::StoryComponent;

use self::story::{StoryComponentAction, story_component_prompt};

#[derive(Debug, Clone)]
pub struct Area {
    pub name: &'static str,
    pub unique_id: &'static str,
    pub story: Vec<StoryComponent>,
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
        while player.story_progress.current_area_progress < self.story.len() {
            let action = self.get_action(player.story_progress.current_area_progress);
            match action {
                StoryComponentAction::ShowText(text) => {
                    println!("{text}");
                    player.story_progress.current_area_progress += 1;
                },
                StoryComponentAction::Battle(enemies) => {
                    if !battle(&player, &enemies) {
                        return AreaResult::PlayerWasDefeated;
                    } else {
                        player.experience.enemies_defeated(&enemies);
                    }
                    println!("{}", enemies.len());
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
                StoryComponentAction::LearnSkill(skill) => {
                    player.skill_ids.push(String::from(skill.unique_id));
                    player.story_progress.current_area_progress += 1;
                },
            }
            save::save(&player);
        }
        // TODO: the following four lines should be run outside this function once it returns as game::area_completed
        player.experience.area_cleared();
        player.story_progress.areas_completed += 1;
        player.story_progress.current_area_progress = 0;
        save::save(&player);
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
            StoryComponent::LearnSkill(skill) => StoryComponentAction::LearnSkill(skill),
        }
    }
}

pub fn build_areas() -> [Area; 2] {
    return [plains::new(), forest::new()]; // TODO: add more areas
}
