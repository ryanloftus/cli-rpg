use super::Area;
use super::StoryComponent;

const INTRO_PROMPT: &str = "You're currently in the throne room of The Kingdom's central castle.
You're not sure why, but you seem to be speaking before the King. He says:
The world is in need of a hero...
Territories far and wide are in disarray wars, monster attacks, and natural disasters.
Will you help save the world? [Y/N]";

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
    return vec![StoryComponent::Text(String::from(INTRO_PROMPT))];
}
