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
pub struct Stats {
    max_health: u16,
    max_mp: u16,
    strength: u16,
    magic: u16,
    defense: u16,
    magic_resist: u16,
    speed: u16,
    skill: u16,
    luck: u16,
}
