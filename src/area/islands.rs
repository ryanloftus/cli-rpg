use crate::unit::enemy::{boss, soldier::SoldierType, Enemy, EnemyType};

use super::StoryComponent;

const OPENING_TEXT: &str = "There are three major islands, the Warm Island, the Cold Island, and the Stormy Island. The three islands have been at war with each other for 100 years. Each is ruled by a tyrannical leader who refuses to give up on the war. It is up to you to end this war quickly by defeating each of the three leaders.";
const CLOSING_TEXT: &str = "The island's leaders have all been defeated and the war is finally over. The people of three islands have formed a democratic society, uniting all three islands. Well done hero.";
const ISLANDS_BASE_LEVEL: u16 = 30;

pub fn story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    story.append(&mut get_warm_island_story());
    story.append(&mut get_cold_island_story());
    story.append(&mut get_stormy_island_story());
    story.push(StoryComponent::Text(String::from(CLOSING_TEXT)));
    return story;
}

fn get_warm_island_story() -> Vec<StoryComponent> {
    let mut warm_island_story = Vec::new();
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        warm_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction: String::from("Warm Island"),
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i,
        )));
    }
    warm_island_story.push(StoryComponent::Boss(boss::warm_island_king()));
    return warm_island_story;
}

fn get_cold_island_story() -> Vec<StoryComponent> {
    let mut cold_island_story = Vec::new();
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        cold_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction: String::from("Cold Island"),
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i + 30,
        )));
    }
    cold_island_story.push(StoryComponent::Boss(boss::cold_island_king()));
    return cold_island_story;
}

fn get_stormy_island_story() -> Vec<StoryComponent> {
    let mut stormy_island_story = Vec::new();
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        stormy_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction: String::from("Stormy Island"),
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i + 60,
        )));
    }
    stormy_island_story.push(StoryComponent::Boss(boss::stormy_island_queen()));
    return stormy_island_story;
}

fn get_soldier_type_at(i: u8) -> SoldierType {
    if i <= 5 {
        SoldierType::Footsoldier
    } else if i <= 10 {
        SoldierType::Archer
    } else if i <= 20 {
        SoldierType::Knight
    } else {
        SoldierType::Guard
    }
}
