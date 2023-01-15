use crate::enemy::Enemy;
use crate::player::Player;

/*
 * Starts a battle between player and enemies.
 * Returns true if the player won and false otherwise.
 */
pub fn battle(mut player: &Player, enemies: Vec<Enemy>) -> bool {
    // TODO: implement this function
    let winner = player.name.clone();
    for enemy in enemies {
        let loser = enemy.name.clone();
        println!("{winner} beat {loser}");
    }
    return true;
}
