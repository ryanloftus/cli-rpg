use super::{story::StoryComponent, Area};
use crate::enemy::{monster::MonsterType, Enemy, EnemyDifficulty, EnemyType};

const OPENING_TEXT: &str = "You enter the forest and notice that 100 citizens of The Forest have been turned into Demons. Defeat the Demons and whoever is responsible for this tragedy!";
const CLOSING_TEXT: &str =
    "The Demon Lord has been defeated and The Forest is safe. Well done hero.";

/*
 * This is the third area in the game.
 * The area follows a simple pattern of fighting enemies until the boss is reached.
 */
pub fn new() -> Area {
    Area {
        name: "The Forest",
        unique_id: "F",
        story: generate_story(),
    }
}

fn generate_story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(Enemy::new(
            enemy_type_at(i),
            15,
            i,
        )));
    }
    story.push(StoryComponent::Boss(Enemy {
        name: String::from("Demon Lord"),
        level: 25,
        skills: Vec::new(),
        difficulty: EnemyDifficulty::Boss,
    }));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn enemy_type_at(story_idx: u8) -> EnemyType {
    return EnemyType::Monster(
        if story_idx % 5 == 0 {
            MonsterType::DemonicBeast
        } else {
            MonsterType::Demon
        }
    );
}
