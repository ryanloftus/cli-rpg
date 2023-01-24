use crate::enemy::{Enemy, EnemyType};

use super::{Area, story::StoryComponent};

const OPENING_TEXT: &str = "There are three major islands, the Warm Island, the Cold Island, and the Stormy Island. The three islands have been at war with each other for 100 years. Each is ruled by a tyrannical leader who refuses to give up on the war. It is up to you to end this war quickly by defeating each of the three leaders.";
const CLOSING_TEXT: &str = "The island's leaders have all been defeated and the war is finally over. The people of three islands have formed a democratic society, uniting all three islands. Well done hero.";

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
    for i in 1..=100 {
        story.push(StoryComponent::Enemy(
            Enemy::new(EnemyType::Soldier { faction: todo!(), soldier_type: todo!() }, , );
        ));
    }
    return story;
}
