use crate::player::Player;
use crate::enemy::Enemy;

pub fn do_battle(mut player: &Player, enemy: &Enemy) {
    // TODO: implement this function
    let winner = player.name.clone();
    let loser = enemy.name.clone();
    println!("{winner} beat {loser}!");
}
