use serde::{Deserialize, Serialize};

/*
 * Stats each unit has. Used for battles.
 *
 * max_health: how much damage a unit can take before they are defeated
 * max_mp: how much magic a unit can use before running out
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
    pub max_mp: u16,
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
    MaxMp(f32),
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
            max_health: base,
            max_mp: base,
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
        stats.max_mp += BASE_MP;
        return stats;
    }

    fn apply_multiplier(&mut self, multiplier: StatMultiplier) {
        match multiplier {
            StatMultiplier::MaxHealth(multiplier) => {
                self.max_health = Self::mult_stat(self.max_health, multiplier)
            }
            StatMultiplier::MaxMp(multiplier) => {
                self.max_mp = Self::mult_stat(self.max_mp, multiplier)
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
pub const BASE_MP: u16 = 10;
