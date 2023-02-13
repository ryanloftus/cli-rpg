use crate::{
    prompt::{get_optional_selection_from_options, get_selection_from_options, PromptOption},
    unit::{enemy::Enemy, player::Player, skill::Skill},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BattleResult {
    Victory,
    Defeat,
    Retreat,
}

#[derive(Debug, Clone)]
enum PlayerTurnAction {
    TacticalRetreat,
    Attack,
    Skill(Skill),
    UnselectedSkill,
}

impl PromptOption for PlayerTurnAction {
    fn option_name(&self) -> String {
        return String::from(match self {
            PlayerTurnAction::TacticalRetreat => "Tactical Retreat",
            PlayerTurnAction::Attack => "Attack",
            PlayerTurnAction::Skill(_) => {
                panic!("Skill turn action should not be used as option directly")
            }
            PlayerTurnAction::UnselectedSkill => "Skill",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        return match self {
            PlayerTurnAction::TacticalRetreat => Some(String::from("R")),
            PlayerTurnAction::Attack => Some(String::from("A")),
            PlayerTurnAction::Skill(_) => None,
            PlayerTurnAction::UnselectedSkill => Some(String::from("S")),
        };
    }
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
    let mut mutable_enemies = enemies.clone();
    'outer: loop {
        // player attacks first
        let num_player_turns = calc_num_player_turns(player, &mutable_enemies);
        for _ in 0..num_player_turns {
            match get_turn_action(player, &mutable_enemies) {
                PlayerTurnAction::TacticalRetreat => break 'outer,
                PlayerTurnAction::Attack => todo!(),
                PlayerTurnAction::Skill(skill) => todo!(),
                PlayerTurnAction::UnselectedSkill => panic!("cannot perform unselected skill"),
            }
            let mut i = 0;
            while i < mutable_enemies.len() {
                if enemy_health[i] == 0 {
                    mutable_enemies.remove(i);
                    enemy_health.remove(i);
                } else {
                    i += 1;
                }
            }
        }
        // perform enemy turns
        for enemy in &mutable_enemies {
            let damage_to_player = calc_enemy_attack_damage(enemy, player);
            player_health -= damage_to_player;
            if player_health == 0 {
                break 'outer;
            }
        }
    }
    if player_health == 0 {
        print_defeat_message(player, enemies);
        return BattleResult::Defeat;
    } else if mutable_enemies.is_empty() {
        print_victory_message(player, enemies);
        return BattleResult::Victory;
    } else {
        print_retreat_message(player);
        return BattleResult::Retreat;
    }
}

fn get_turn_action(player: &Player, enemies: &Vec<Enemy>) -> PlayerTurnAction {
    loop {
        println!("You are attacking {}", enemies[0].name);
        let mut turn_action = get_selection_from_options(
            String::from("What will you do?"),
            &vec![
                PlayerTurnAction::Attack,
                PlayerTurnAction::UnselectedSkill,
                PlayerTurnAction::TacticalRetreat,
            ],
        );
        if let PlayerTurnAction::UnselectedSkill = turn_action {
            let skill = get_optional_selection_from_options(
                String::from("Which skill will you use?"),
                &player.skills,
            );
            if let Some(skill) = skill {
                turn_action = PlayerTurnAction::Skill(skill);
            } else {
                continue;
            }
        }
        return turn_action;
    }
}

fn calc_num_player_turns(player: &Player, enemies: &Vec<Enemy>) -> usize {
    let enemy_speed: u16 =
        enemies.iter().map(|e| e.stats.speed).sum::<u16>() / enemies.len() as u16;
    let player_speed = player.stats.speed;
    let mut num_turns = 1;
    while player_speed > enemy_speed * 2 {
        num_turns += 1;
    }
    return num_turns;
}

fn calc_enemy_attack_damage(enemy: &Enemy, player: &Player) -> u16 {
    todo!()
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

fn print_retreat_message(player: &Player) {
    println!("{} retreated successfully.", player.name);
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
