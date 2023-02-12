use crate::unit::{enemy::Enemy, player::Player, skill::Skill};

pub enum BattleResult {
    Victory,
    Defeat,
}

pub enum PlayerTurnAction {
    TacticalRetreat,
    Attack,
    Skill(Skill),
}

/*
 * Starts a battle between player and enemies.
 * Returns true if the player won and false otherwise.
 */
pub fn battle(player: &mut Player, enemies: &Vec<Enemy>) -> BattleResult {
    if enemies[0].name == "Doom Incarnate" {
        return final_boss_battle(player);
    }
    let mut player_health = player.stats.max_health;
    let mut enemy_health: Vec<u16> = enemies.iter().map(|e| e.stats.max_health).collect();
    // TODO: implement actual battle logic
    // TODO: special battle logic for final boss
    print_victory_message(player, enemies);
    return BattleResult::Victory;
}

fn print_defeat_message(player: &Player, enemies: &Vec<Enemy>) {
    if enemies.iter().find(|e| e.name == "Shrek").is_some() {
        println!("{}\n{}", ASCII_SHREK, "GAME OGRE");
    } else {
        println!("{} was defeated.", player.name);
    }
}

fn print_victory_message(player: &Player, enemies: &Vec<Enemy>) {
    for enemy in enemies {
        println!("{} defeated {}!", player.name, enemy.name);
    }
}

fn final_boss_battle(player: &Player) -> BattleResult {
    println!("You throw every attack you have at Doom Incarnate, but nothing seems to affect the shadowy being...");
    println!("It would seem somethings cannot be solved with violence...");
    println!(
        "In a panic you begin going through your pockets in the hopes of finding something useful."
    );
    println!("You find the strange rock you picked up in The Kingdom and decide to throw it into the fog...");
    println!("Doom Incarnate:");
    println!("\"Hey, I've been looking for this thing for centuries! I dropped it so long ago I thought I'd never see it again.");
    println!("Thanks {}, I guess I misjudged you. I'll go back to sleep for a couple centuries and wait until you're long gone before I come back and destroy the world.", player.name);
    return BattleResult::Victory;
}

const ASCII_SHREK: &str = "⢀⡴⠑⡄⠀⠀⠀⠀⠀⠀⠀⣀⣀⣤⣤⣤⣀⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 
⠸⡇⠀⠿⡀⠀⠀⠀⣀⡴⢿⣿⣿⣿⣿⣿⣿⣿⣷⣦⡀⠀⠀⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠑⢄⣠⠾⠁⣀⣄⡈⠙⣿⣿⣿⣿⣿⣿⣿⣿⣆⠀⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⢀⡀⠁⠀⠀⠈⠙⠛⠂⠈⣿⣿⣿⣿⣿⠿⡿⢿⣆⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⢀⡾⣁⣀⠀⠴⠂⠙⣗⡀⠀⢻⣿⣿⠭⢤⣴⣦⣤⣹⠀⠀⠀⢀⢴⣶⣆ 
⠀⠀⢀⣾⣿⣿⣿⣷⣮⣽⣾⣿⣥⣴⣿⣿⡿⢂⠔⢚⡿⢿⣿⣦⣴⣾⠁⠸⣼⡿ 
⠀⢀⡞⠁⠙⠻⠿⠟⠉⠀⠛⢹⣿⣿⣿⣿⣿⣌⢤⣼⣿⣾⣿⡟⠉⠀⠀⠀⠀⠀ 
⠀⣾⣷⣶⠇⠀⠀⣤⣄⣀⡀⠈⠻⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀ 
⠀⠉⠈⠉⠀⠀⢦⡈⢻⣿⣿⣿⣶⣶⣶⣶⣤⣽⡹⣿⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⠀⠉⠲⣽⡻⢿⣿⣿⣿⣿⣿⣿⣷⣜⣿⣿⣿⡇⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⠀⠀⢸⣿⣿⣷⣶⣮⣭⣽⣿⣿⣿⣿⣿⣿⣿⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⣀⣀⣈⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠇⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⢿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⠃⠀⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⠀⠹⣿⣿⣿⣿⣿⣿⣿⣿⣿⣿⡿⠟⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀ 
⠀⠀⠀⠀⠀⠀⠀⠀⠀⠉⠛⠻⠿⠿⠿⠿⠛⠉";
