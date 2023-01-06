use crate::enemy::Enemy;

#[derive(Debug, Clone)]
pub enum StoryComponent {
    Enemy(Enemy),
    Boss(Enemy),
    Text(String),
    // TODO: add complex component that takes the story vector to allow for dynamic stories (choose your path)
}
