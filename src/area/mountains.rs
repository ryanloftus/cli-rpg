use super::StoryComponent;
use crate::unit::enemy::{
    boss, monster::MonsterType, soldier::SoldierType, Enemy, EnemyAttribute, EnemyType,
};

const OPENING_TEXT: &str = "You now enter the final region of your journey, The Mountains. You begin your climb up the tallest mountain, but your view forward is obscured by a thick black fog. You march onward into the unknown to face your destiny.";
const CLOSING_TEXT: &str = "You have put a stop to Doom Incarnate's malicious machinations... or at least delayed them. The black fog shrouding the mountains has cleared and you look down on the world you've protected. Well done hero, you may finally rest.";
const ENEMY_PREFIX: &str = "Shadow";

pub fn story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    for i in 1..=100 {
        let mut next_enemy = Enemy::new(enemy_type_at(i), 75, i);
        next_enemy.attributes.push(EnemyAttribute::Dark);
        story.push(StoryComponent::Enemy(next_enemy));
    }
    story.push(StoryComponent::Boss(boss::doom_incarnate()));
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn enemy_type_at(story_idx: u8) -> EnemyType {
    return if story_idx % 3 == 0 {
        EnemyType::Soldier {
            faction: String::from(ENEMY_PREFIX),
            soldier_type: if story_idx >= 60 {
                if story_idx % 2 == 0 {
                    SoldierType::Lieutenant
                } else {
                    SoldierType::Guard
                }
            } else if story_idx % 5 == 0 {
                SoldierType::Knight
            } else if story_idx % 2 == 0 {
                SoldierType::Archer
            } else {
                SoldierType::Footsoldier
            },
        }
    } else {
        EnemyType::Monster {
            name_prefix: Some(String::from(ENEMY_PREFIX)),
            monster_type: match story_idx % 4 {
                0 => MonsterType::Goblin,
                1 => MonsterType::Ogre,
                2 => MonsterType::Dragon,
                _ => MonsterType::Slime,
            },
        }
    };
}
