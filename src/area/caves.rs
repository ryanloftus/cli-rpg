use super::{story::StoryComponent, Area};
use crate::enemy::{soldier::SoldierType, Enemy, EnemyDifficulty, EnemyType};

const OPENING_TEXT: &str = "You enter Mole City in The Caves and speak to The Moleman, leader of the Mole People. He tells you that his people and territory are being ruthlessly attacked by the Lizard People. Defend Mole City from the Lizard People.";
const CLOSING_TEXT: &str =
    "The Lizard King has been defeated and Mole City is once again safe. Well done hero.";

pub fn new() -> Area {
    Area {
        name: "The Caves",
        unique_id: "C",
        story: generate_story(),
    }
}

fn generate_story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(Enemy::new(enemy_type_at(i), 45, i)));
    }
    story.push(StoryComponent::Boss(Enemy {
        name: String::from("Lizard King"),
        level: 60,
        skills: Vec::new(),
        difficulty: EnemyDifficulty::Boss,
    }));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn enemy_type_at(story_idx: u8) -> EnemyType {
    return EnemyType::Soldier {
        faction: String::from("Lizardmen"),
        soldier_type: if story_idx % 10 == 0 {
            SoldierType::Lieutenant
        } else if story_idx >= 80 {
            SoldierType::Guard
        } else if story_idx % 5 == 0 {
            SoldierType::Knight
        } else if story_idx % 3 == 0 {
            SoldierType::Archer
        } else {
            SoldierType::Footsoldier
        },
    };
}
