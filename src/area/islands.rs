use crate::enemy::{soldier::SoldierType, Enemy, EnemyDifficulty, EnemyType};

use super::{story::StoryComponent, Area};

const OPENING_TEXT: &str = "There are three major islands, the Warm Island, the Cold Island, and the Stormy Island. The three islands have been at war with each other for 100 years. Each is ruled by a tyrannical leader who refuses to give up on the war. It is up to you to end this war quickly by defeating each of the three leaders.";
const CLOSING_TEXT: &str = "The island's leaders have all been defeated and the war is finally over. The people of three islands have formed a democratic society, uniting all three islands. Well done hero.";
const ISLANDS_BASE_LEVEL: u8 = 30;

pub fn new() -> Area {
    Area {
        name: "The Islands",
        unique_id: "I",
        story: generate_story(),
    }
}

fn generate_story() -> Vec<StoryComponent> {
    let mut story = Vec::new();
    story.push(StoryComponent::Text(String::from(OPENING_TEXT)));
    story.append(&mut get_warm_island_story());
    story.append(&mut get_cold_island_story());
    story.append(&mut get_stormy_island_story());
    return story;
}

fn get_warm_island_story() -> Vec<StoryComponent> {
    let warm_island_story = Vec::new();
    let faction = String::from("Warm Island");
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        warm_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction,
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i,
        )));
    }
    warm_island_story.push(StoryComponent::Boss(Enemy {
        name: String::from("Warm Island King"),
        skills: Vec::new(),
        level: 34,
        difficulty: EnemyDifficulty::Boss,
    }));
    return warm_island_story;
}

fn get_cold_island_story() -> Vec<StoryComponent> {
    let cold_island_story = Vec::new();
    let faction = String::from("Cold Island");
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        cold_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction,
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i + 30,
        )));
    }
    cold_island_story.push(StoryComponent::Boss(Enemy {
        name: String::from("Cold Island King"),
        skills: Vec::new(),
        level: 38,
        difficulty: EnemyDifficulty::Boss,
    }));
    return cold_island_story;
}

fn get_stormy_island_story() -> Vec<StoryComponent> {
    let stormy_island_story = Vec::new();
    let faction = String::from("Stormy Island");
    for i in 1..=30 {
        let soldier_type = get_soldier_type_at(i);
        stormy_island_story.push(StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction,
                soldier_type,
            },
            ISLANDS_BASE_LEVEL,
            i + 60,
        )));
    }
    stormy_island_story.push(StoryComponent::Boss(Enemy {
        name: String::from("Stormy Island Queen"),
        skills: Vec::new(),
        level: 42,
        difficulty: EnemyDifficulty::Boss,
    }));
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
