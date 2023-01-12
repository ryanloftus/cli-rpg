use crate::player::Player;
use crate::enemy::Enemy;

pub fn do_battle(mut player: &Player, enemies: Vec<Enemy>) {
    // TODO: implement this function
    let winner = player.name.clone();
    for enemy in enemies {
        let loser = enemy.name.clone();
        println!("{winner} beat {loser}");
    }
}
