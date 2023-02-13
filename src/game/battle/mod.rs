use crate::{
    prompt::{get_optional_selection_from_options, get_selection_from_options, PromptOption},
    unit::{
        enemy::{Enemy, EnemyAttribute},
        player::Player,
        skill::{Skill, SkillAttribute},
        stats::Stats,
    },
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
pub fn battle(_player: &mut Player, _enemies: &Vec<Enemy>) -> BattleResult {
    if _enemies[0].name == "Doom Incarnate" {
        return final_boss_battle(_player);
    }
    let mut player = player_to_player_in_battle(_player);
    let mut enemies = _enemies.iter().map(enemy_to_enemy_in_battle).collect();
    'outer: loop {
        // perform player turns
        let num_player_turns = calc_num_player_turns(&player, &enemies);
        for _ in 0..num_player_turns {
            match get_turn_action(&player, &enemies) {
                PlayerTurnAction::TacticalRetreat => break 'outer,
                PlayerTurnAction::Attack => perform_player_attack(&player, &mut enemies[0]),
                PlayerTurnAction::Skill(skill) => {
                    _player.skills.iter_mut().for_each(|s| {
                        if s.skill_type == skill.skill_type {
                            s.experience.skill_used();
                        }
                    });
                    perform_player_skill(&mut player, &mut enemies, &skill);
                }
                PlayerTurnAction::UnselectedSkill => panic!("cannot perform unselected skill"),
            }
            enemies = filter_defeated_enemies(enemies, &player);
            if enemies.is_empty() {
                break 'outer;
            }
        }
        // perform enemy turns
        for enemy in &enemies {
            perform_enemy_attack(enemy, &mut player);
            if player.health == 0 {
                break 'outer;
            }
        }
        // apply DOT effects
        for enemy in &mut enemies {
            enemy.health = apply_damage(enemy.health, enemy.dot_per_turn);
        }
        enemies = filter_defeated_enemies(enemies, &player);
        if enemies.is_empty() {
            break 'outer;
        }
    }
    if player.health == 0 {
        print_defeat_message(&player, &enemies);
        return BattleResult::Defeat;
    } else if enemies.is_empty() {
        print_victory_message(&player);
        return BattleResult::Victory;
    } else {
        print_retreat_message(&player);
        return BattleResult::Retreat;
    }
}

fn get_turn_action(player: &PlayerInBattle, enemies: &Vec<EnemyInBattle>) -> PlayerTurnAction {
    let mut action_options = vec![PlayerTurnAction::Attack, PlayerTurnAction::TacticalRetreat];
    if !player.skills.is_empty() {
        action_options.push(PlayerTurnAction::UnselectedSkill);
    }
    loop {
        println!("You are attacking {}.", enemies[0].name);
        let mut turn_action =
            get_selection_from_options(String::from("What will you do?"), &action_options);
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

fn filter_defeated_enemies(
    enemies: Vec<EnemyInBattle>,
    player: &PlayerInBattle,
) -> Vec<EnemyInBattle> {
    return enemies
        .into_iter()
        .filter(|e| {
            let is_defeated = e.health == 0;
            if is_defeated {
                println!("{} defeated {}!", player.name, e.name);
            }
            !is_defeated
        })
        .collect();
}

fn perform_enemy_attack(enemy: &EnemyInBattle, player: &mut PlayerInBattle) {
    println!("{} attacked!", enemy.name);
    let mut bonus_defense_stat = player.bonus_defense as f32;
    if enemy.stats.magic > enemy.stats.strength {
        bonus_defense_stat = player.bonus_magic_resist as f32;
    }
    let damage = (calc_attack_damage(&enemy.stats, &player.stats) as f32
        / (bonus_defense_stat + 1.0)) as u16;
    player.health -= apply_damage(damage, player.health);
    print_damage_message(damage, &player.name);
}

fn perform_player_attack(player: &PlayerInBattle, enemy: &mut EnemyInBattle) {
    println!("{} attacked!", player.name);
    let damage = calc_attack_damage(&player.stats, &enemy.stats);
    enemy.health -= apply_damage(damage, enemy.health);
    print_damage_message(damage, &enemy.name);
}

fn perform_player_skill(
    player: &mut PlayerInBattle,
    enemies: &mut Vec<EnemyInBattle>,
    skill: &Skill,
) {
    println!("{} used {}!", player.name, skill.option_name());
    let enemies_targeted = if skill.has_attribute(SkillAttribute::AreaOfEffect) {
        0..(5 * skill.experience.level as usize)
    } else {
        0..1
    };
    if skill.has_attribute(SkillAttribute::Physical) {
        for i in enemies_targeted {
            let damage = (calc_damage_from_stats(player.stats.strength, enemies[i].stats.defense)
                as f32
                * calc_effectiveness(skill, &enemies[i])
                * (1.0 + skill.experience.level as f32 / 10.0))
                .round() as u16;
            enemies[i].health -= apply_damage(enemies[i].health, damage);
            print_damage_message(damage, &enemies[i].name);
        }
    } else if skill.has_attribute(SkillAttribute::Magic) {
        for i in enemies_targeted {
            let damage = (calc_damage_from_stats(player.stats.magic, enemies[i].stats.magic_resist)
                as f32
                * calc_effectiveness(skill, &enemies[i])
                * (1.0 + skill.experience.level as f32 / 10.0))
                .round() as u16;
            enemies[i].health -= apply_damage(enemies[i].health, damage);
            print_damage_message(damage, &enemies[i].name);
        }
    }
    if skill.has_attribute(SkillAttribute::Healing) {
        let heal = std::cmp::min(
            player.stats.magic * skill.experience.level / 4,
            player.stats.max_health - player.health,
        );
        player.health += heal;
        println!("{} healed for {} health!", player.name, heal);
    }
    if skill.has_attribute(SkillAttribute::MagicResistive) {
        player.bonus_magic_resist += skill.experience.level;
        println!("{} gained magic resistance!", player.name);
    }
    if skill.has_attribute(SkillAttribute::Defensive) {
        player.bonus_defense += skill.experience.level;
        println!("{} gained defense!", player.name);
    }
}

fn calc_num_player_turns(player: &PlayerInBattle, enemies: &Vec<EnemyInBattle>) -> usize {
    let enemy_speed: u16 =
        enemies.iter().map(|e| e.stats.speed).sum::<u16>() / enemies.len() as u16;
    let mut player_speed = player.stats.speed;
    let mut num_turns = 1;
    while player_speed > enemy_speed * 2 {
        num_turns += 1;
        player_speed /= 2;
    }
    return num_turns;
}

fn calc_attack_damage(attacker_stats: &Stats, defender_stats: &Stats) -> u16 {
    let mut attack_stat = attacker_stats.strength;
    let mut defense_stat = defender_stats.defense;
    if attacker_stats.magic > attacker_stats.strength {
        attack_stat = attacker_stats.magic;
        defense_stat = defender_stats.magic_resist;
    }
    return calc_damage_from_stats(attack_stat, defense_stat);
}

fn calc_effectiveness(skill: &Skill, enemy: &EnemyInBattle) -> f32 {
    let mut effectiveness: f32 = 1.0;
    for attribute in &enemy.attributes {
        match attribute {
            EnemyAttribute::Hot => {
                if skill.has_attribute(SkillAttribute::Ice)
                    || skill.has_attribute(SkillAttribute::Water)
                {
                    effectiveness *= 2.0;
                } else if skill.has_attribute(SkillAttribute::Fire) {
                    effectiveness *= 0.5;
                }
            }
            EnemyAttribute::Cold => {
                if skill.has_attribute(SkillAttribute::Fire) {
                    effectiveness *= 2.0;
                } else if skill.has_attribute(SkillAttribute::Ice) {
                    effectiveness *= 0.5
                }
            }
            EnemyAttribute::Dark => {
                if skill.has_attribute(SkillAttribute::Light) {
                    effectiveness *= 2.0;
                }
            }
        }
    }
    return effectiveness;
}

fn calc_damage_from_stats(attack_stat: u16, defense_stat: u16) -> u16 {
    let attack_power = attack_stat / 2;
    let defense_power = std::cmp::min(defense_stat / 4, attack_power);
    return attack_power - defense_power;
}

fn apply_damage(health: u16, damage: u16) -> u16 {
    return std::cmp::min(health, damage);
}

fn print_damage_message(damage: u16, name: &str) {
    println!("{} took {} damage!", name, damage);
}

fn print_defeat_message(player: &PlayerInBattle, enemies: &Vec<EnemyInBattle>) {
    if enemies.iter().find(|e| e.name == "Shrek").is_some() {
        println!("{}\n{}", ASCII_SHREK, "GAME OGRE");
    } else {
        println!("{} was defeated.", player.name);
    }
}

fn print_victory_message(player: &PlayerInBattle) {
    println!("{} won the battle!", player.name);
}

fn print_retreat_message(player: &PlayerInBattle) {
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
    println!("Thanks {}, I guess I misjudged you. I'll go back to sleep for a couple centuries and wait until you're long gone before I come back and destroy the world.\"", player.name);
    return BattleResult::Victory;
}

struct EnemyInBattle {
    name: String,
    health: u16,
    stats: Stats,
    attributes: Vec<EnemyAttribute>,
    dot_per_turn: u16,
}

fn enemy_to_enemy_in_battle(enemy: &Enemy) -> EnemyInBattle {
    return EnemyInBattle {
        name: enemy.name.clone(),
        health: enemy.stats.max_health,
        stats: enemy.stats.clone(),
        attributes: enemy.attributes.clone(),
        dot_per_turn: 0,
    };
}

struct PlayerInBattle {
    name: String,
    health: u16,
    stats: Stats,
    skills: Vec<Skill>,
    bonus_defense: u16,
    bonus_magic_resist: u16,
}

fn player_to_player_in_battle(player: &Player) -> PlayerInBattle {
    return PlayerInBattle {
        name: player.name.clone(),
        health: player.stats.max_health,
        stats: player.stats.clone(),
        skills: player.skills.clone(),
        bonus_defense: 0,
        bonus_magic_resist: 0,
    };
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
