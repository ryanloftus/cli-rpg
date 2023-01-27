mod area;
mod battle;
mod enemy;
mod game;
mod player;
mod progression;
mod prompt;
mod save;
mod skill;
mod stats;

// TODO: add colored text to terminal output
fn main() {
    game::play_game();
}
