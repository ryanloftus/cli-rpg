use crate::enemy::{EnemyType, Enemy, monster::MonsterType};
use super::{Area, story::StoryComponent};

const OPENING_TEXT: &str = "";
const CLOSING_TEXT: &str = "";

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
    for i in 1..100 {
        story.push(StoryComponent::Enemy(
            if i % 20 == 0 {
                Enemy::new(EnemyType::Monster(MonsterType::Dragon), (i / 20) + 1, (i / 15) + 1)
            } else if i % 10 == 0 {
                Enemy::new(EnemyType::Monster(MonsterType::Ogre), (i / 20) + 1, (i / 15) + 1)
            } else if i % 3 == 0 {
                Enemy::new(EnemyType::Monster(MonsterType::Goblin), 1, (i / 25) + 1)
            } else {
                Enemy::new(EnemyType::Monster(MonsterType::Slime), 1, (i / 25) + 1)
            }
        ))
    }
    story.push(StoryComponent::Boss(
        Enemy {
            name: String::from("Demon Lord"),
            level: 10,
            skills: Vec::new(),
        }
    ));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    story
}
