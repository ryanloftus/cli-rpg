use rand::Rng;
use serde::{Deserialize, Serialize};

/*
 * Stats each unit has. Used for battles.
 *
 * max_health: how much damage a unit can take before they are defeated
 * strength: increases damage dealt by physical skills
 * magic: increases damage/healing done by magic skills
 * defense: decreases damage received from physical skills
 * magic_resist: decreases damage received from magic skills
 * speed: increases the frequency of the unit's turns relative to their opponent
 * skill: increases hit chance, parry chance, and critical hit chance
 * luck: ???
 */
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub max_health: u16,
    pub strength: u16,
    pub magic: u16,
    pub defense: u16,
    pub magic_resist: u16,
    pub speed: u16,
    pub skill: u16,
    pub luck: u16,
}

pub enum StatMultiplier {
    MaxHealth(f32),
    Strength(f32),
    Magic(f32),
    Defense(f32),
    MagicResist(f32),
    Speed(f32),
    Skill(f32),
    Luck(f32),
}

impl Stats {
    /*
     * Creates a new Stats scaled to the given values
     * in most cases, base should be the level of the unit
     * stat_multiplers can be passed to increase particular stats
     */
    pub fn new(base: u16, stat_multipliers: Vec<StatMultiplier>) -> Stats {
        let mut stats = Stats {
            max_health: base * BASE_HEALTH_MULTIPLIER,
            strength: base,
            magic: base,
            defense: base,
            magic_resist: base,
            speed: base,
            skill: base,
            luck: base,
        };
        for multiplier in stat_multipliers {
            stats.apply_multiplier(multiplier);
        }
        // we don't want the multiplier to apply to base stats given to all units at level 1, so we add them here
        stats.max_health += BASE_HEALTH;
        return stats;
    }

    pub fn level_up(&mut self, stat_multipliers: Vec<StatMultiplier>) {
        self.max_health += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::MaxHealth(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.strength += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Strength(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.magic += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Magic(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.defense += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Defense(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.magic_resist += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::MagicResist(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.speed += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Speed(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.skill += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Skill(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
        self.luck += Self::calc_stat_gain(stat_multipliers.iter().find_map(|m| {
            if let StatMultiplier::Luck(mult) = m {
                Some(mult.clone())
            } else {
                None
            }
        }));
    }

    fn calc_stat_gain(stat_multiplier: Option<f32>) -> u16 {
        if let Some(mut mult) = stat_multiplier {
            let mut stat_gain = 0;
            while mult > 1.0 {
                stat_gain += 1;
                mult -= 1.0;
            }
            if rand::thread_rng().gen_range(0.0..1.0) < mult {
                stat_gain += 1;
            }
            return stat_gain;
        } else {
            return 1;
        }
    }

    fn apply_multiplier(&mut self, multiplier: StatMultiplier) {
        match multiplier {
            StatMultiplier::MaxHealth(multiplier) => {
                self.max_health = Self::mult_stat(self.max_health, multiplier)
            }
            StatMultiplier::Strength(multiplier) => {
                self.strength = Self::mult_stat(self.strength, multiplier)
            }
            StatMultiplier::Magic(multiplier) => {
                self.magic = Self::mult_stat(self.magic, multiplier)
            }
            StatMultiplier::Defense(multiplier) => {
                self.defense = Self::mult_stat(self.defense, multiplier)
            }
            StatMultiplier::MagicResist(multiplier) => {
                self.magic_resist = Self::mult_stat(self.magic_resist, multiplier)
            }
            StatMultiplier::Speed(multiplier) => {
                self.speed = Self::mult_stat(self.speed, multiplier)
            }
            StatMultiplier::Skill(multiplier) => {
                self.skill = Self::mult_stat(self.skill, multiplier)
            }
            StatMultiplier::Luck(multiplier) => self.luck = Self::mult_stat(self.luck, multiplier),
        }
    }

    fn mult_stat(stat: u16, multiplier: f32) -> u16 {
        return (f32::from(stat) * multiplier).round() as u16;
    }
}

pub const BASE_HEALTH: u16 = 20;
pub const BASE_HEALTH_MULTIPLIER: u16 = 2; // Health should grow at roughly double the rate of all other stats
