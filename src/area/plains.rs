use super::{Area, StoryComponent};
use crate::enemy::{monster::MonsterType, Enemy, EnemyType};

const OPENING_TEXT: &str = "You look out onto The Plains and see that it has been overrun by 100 Monsters. Its up to you to stop them!";
const CLOSING_TEXT: &str =
    "The Villainous Mage has been defeated and The Plains are safe. Well done hero.";

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
            EnemyType::Monster {
                name_prefix: None,
                monster_type: if i % 20 == 0 {
                    MonsterType::Dragon
                } else if i % 10 == 0 {
                    MonsterType::Ogre
                } else if i % 3 == 0 {
                    MonsterType::Goblin
                } else {
                    MonsterType::Slime
                },
            },
            1,
            i,
        )));
    }
    story.push(StoryComponent::Boss(Enemy::new_boss(
        String::from("Villainous Mage"),
        15,
        Vec::new(),
        Vec::new(),
    )));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}
