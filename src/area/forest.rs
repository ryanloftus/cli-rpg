use super::StoryComponent;
use crate::enemy::{monster::MonsterType, Enemy, EnemyType};

const OPENING_TEXT: &str = "You enter The Forest and notice that 100 citizens of The Forest have been turned into Demons. Defeat the Demons and whoever is responsible for this tragedy!";
const CLOSING_TEXT: &str =
    "The Demon Lord has been defeated and The Forest is safe. Well done hero.";

pub fn story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(Enemy::new(enemy_type_at(i), 15, i)));
    }
    story.push(StoryComponent::Boss(Enemy::new_boss(
        String::from("Demon Lord"),
        25,
        Vec::new(),
        Vec::new(),
    )));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn enemy_type_at(story_idx: u8) -> EnemyType {
    return EnemyType::Monster {
        name_prefix: None,
        monster_type: if story_idx % 5 == 0 {
            MonsterType::DemonicBeast
        } else {
            MonsterType::Demon
        },
    };
}
