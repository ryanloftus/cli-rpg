use crate::enemy::Enemy;
use crate::player::Player;

/*
 * Starts a battle between player and enemies.
 * Returns true if the player won and false otherwise.
 */
pub fn battle(mut player: &Player, enemies: &Vec<Enemy>) -> bool {
    // TODO: implement actual battle logic
    // TODO: to return more information in the future, return a tuple instead of a bool
    let winner = player.name.clone();
    for enemy in enemies {
        let loser = enemy.name.clone();
        println!("{winner} beat {loser}");
    }
    return true;
}
