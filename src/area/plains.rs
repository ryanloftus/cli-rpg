use super::{story::StoryComponent, Area};
use crate::enemy::{monster::MonsterType, Enemy, EnemyDifficulty, EnemyType};

const OPENING_TEXT: &str = "You look out onto The Plains and see that it has been overrun by 100 Monsters. Its up to you to stop them!";
const CLOSING_TEXT: &str =
    "The Demon Lord has been defeated and The Plains are safe. Well done hero.";

/*
 * This is the second area in the game, and the first where the player fights enemies.
 * The area follows a simple pattern of fighting enemies until the boss is reached.
 */
pub fn new() -> Area {
    Area {
        name: "The Plains",
        unique_id: "P",
        story: generate_story(),
    }
}

fn generate_story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Monster(if i % 20 == 0 {
                MonsterType::Dragon
            } else if i % 10 == 0 {
                MonsterType::Ogre
            } else if i % 3 == 0 {
                MonsterType::Goblin
            } else {
                MonsterType::Slime
            }),
            1,
            i,
        )));
    }
    story.push(StoryComponent::Boss(Enemy {
        name: String::from("Demon Lord"),
        level: 12,
        skills: Vec::new(),
        difficulty: EnemyDifficulty::Boss,
    }));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}
