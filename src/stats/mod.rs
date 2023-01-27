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
