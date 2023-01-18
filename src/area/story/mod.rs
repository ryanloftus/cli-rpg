use crate::{enemy::Enemy, prompt::InputPrompt, skill::Skill};

#[derive(Debug, Clone)]
pub enum StoryComponent<T> {
    Enemy(Enemy),
    Boss(Enemy),
    Text(String),
    Prompt(InputPrompt<T>),
    LearnSkill(&'static Skill),
    // TODO: GainAttribute once player attributes are added
}
