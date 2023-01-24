use crate::enemy::{soldier::SoldierType, Enemy, EnemyType};

use super::Area;
use super::StoryComponent;

const OPENING_TEXT: &str = "You're currently in the throne room of The Kingdom's central castle.
You're not sure why, but you seem to be speaking before the King. He says:
The world is in need of a hero...
Territories far and wide are in disarray wars, monster attacks, and natural disasters.
It's up to you to save the world.";

const TRAINING_BATTLE_TEXT: &str = "First, you should do some training. Go to the training grounds and find a Knight to spar with.";

const CLOSING_TEXT: &str = "Now you are ready, go on to The Plains to start your journey, hero.";

/*
 * This is the first area in the game and acts as a tutorial.
 * The player leaves this area to begin their quest and may return at any time to train.
 */
pub fn new() -> Area {
    Area {
        name: "The Kingdom",
        unique_id: "K",
        story: generate_story(),
    }
}

fn generate_story() -> Vec<StoryComponent> {
    return vec![
        StoryComponent::Text(String::from(OPENING_TEXT)),
        StoryComponent::Text(String::from(TRAINING_BATTLE_TEXT)),
        StoryComponent::Enemy(Enemy::new(
            EnemyType::Soldier {
                faction: String::from("Kingdom"),
                soldier_type: SoldierType::Knight,
            },
            1,
            0,
        )),
        StoryComponent::Text(String::from(CLOSING_TEXT)),
    ];
}
