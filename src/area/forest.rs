use crate::enemy::{EnemyType, Enemy, monster::MonsterType, EnemyDifficulty};
use super::{Area, story::StoryComponent};

const OPENING_TEXT: &str = ""; // TODO
const CLOSING_TEXT: &str = ""; // TODO

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
        story.push(StoryComponent::Enemy(
            if i % 5 == 0 {
                Enemy::new(EnemyType::Monster(MonsterType::DemonicBeast), 10 + (i / 20), 12 + (i / 15))
            } else {
                Enemy::new(EnemyType::Monster(MonsterType::Demon), 10, (i / 20) + 10)
            }
        ))
    }
    story.push(StoryComponent::Boss(
        Enemy {
            name: String::from("Demon Lord"),
            level: 25,
            skills: Vec::new(),
            difficulty: EnemyDifficulty::Boss,
        }
    ));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}
