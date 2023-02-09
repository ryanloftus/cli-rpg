use super::StoryComponent;
use crate::unit::enemy::{boss, soldier::SoldierType, Enemy, EnemyType};

const OPENING_TEXT: &str = "An army of fiery monsters threaten to melt away The Icefields. You must stop this catastrophe and whoever is behind it.";
const CLOSING_TEXT: &str =
    "The volcanic villains have been defeated and The Icefields remain frozen. Well done hero.";

pub fn story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(Enemy::new(enemy_type_at(i), 60, i)));
    }
    story.push(StoryComponent::Boss(boss::molten_monstrosity()));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn enemy_type_at(story_idx: u8) -> EnemyType {
    return EnemyType::Soldier {
        faction: String::from("Volcanic"),
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
