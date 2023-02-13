use super::experience::Experience;
use crate::prompt::PromptOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub skill_type: SkillType,
    pub experience: Experience,
    pub attributes: Vec<SkillAttribute>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillType {
    FireMagic,
    WaterMagic,
    LightningMagic,
    IceMagic,
    ExplosionMagic,
    WindMagic,
    DarkMagic,
    UnholyCurse,
    LightMagic,
    DivineBlessing,
    WeatherForecast,
    SwordStrike,
    DuelStrike,
    DefensiveForm,
    SwordSummoning,
    TempestStrike,
    FlamingStrike,
    FrozenStrike,
    PiercingStrike,
    LanceOfLight,
    Shield,
    DarkSpear,
    DeathFromAbove,
}

impl Skill {
    pub fn new(skill_type: SkillType) -> Skill {
        return Skill {
            skill_type,
            experience: Experience::new(),
            attributes: skill_type.base_attributes(),
        };
    }

    pub fn has_attribute(&self, attribute: SkillAttribute) -> bool {
        return self.attributes.contains(&attribute);
    }
}

impl SkillType {
    fn base_attributes(&self) -> Vec<SkillAttribute> {
        return match self {
            SkillType::FireMagic => vec![SkillAttribute::Magic, SkillAttribute::Fire],
            SkillType::WaterMagic => vec![SkillAttribute::Magic, SkillAttribute::Water],
            SkillType::LightningMagic => vec![SkillAttribute::Magic, SkillAttribute::Electric],
            SkillType::IceMagic => vec![SkillAttribute::Magic, SkillAttribute::Ice],
            SkillType::ExplosionMagic => {
                vec![
                    SkillAttribute::Magic,
                    SkillAttribute::Fire,
                    SkillAttribute::AreaOfEffect,
                ]
            }
            SkillType::WindMagic => vec![SkillAttribute::Magic, SkillAttribute::Wind],
            SkillType::DarkMagic => vec![SkillAttribute::Magic, SkillAttribute::Dark],
            SkillType::UnholyCurse => {
                vec![
                    SkillAttribute::Magic,
                    SkillAttribute::Dark,
                    SkillAttribute::DamageOverTime,
                ]
            }
            SkillType::LightMagic => vec![
                SkillAttribute::Magic,
                SkillAttribute::Light,
                SkillAttribute::Healing,
            ],
            SkillType::DivineBlessing => {
                vec![SkillAttribute::Healing, SkillAttribute::MagicResistive]
            }
            SkillType::WeatherForecast => vec![
                SkillAttribute::Magic,
                SkillAttribute::Water,
                SkillAttribute::Fire,
                SkillAttribute::Wind,
                SkillAttribute::Electric,
                SkillAttribute::Ice,
                SkillAttribute::AreaOfEffect,
            ],
            SkillType::SwordStrike => vec![SkillAttribute::Physical],
            SkillType::DuelStrike => vec![SkillAttribute::Physical, SkillAttribute::MultiHit(2)],
            SkillType::DefensiveForm => vec![SkillAttribute::Defensive],
            SkillType::SwordSummoning => vec![
                SkillAttribute::Magic,
                SkillAttribute::AreaOfEffect,
                SkillAttribute::MultiHit(5),
            ],
            SkillType::TempestStrike => vec![SkillAttribute::Physical, SkillAttribute::Wind],
            SkillType::FlamingStrike => vec![SkillAttribute::Physical, SkillAttribute::Fire],
            SkillType::FrozenStrike => vec![SkillAttribute::Physical, SkillAttribute::Ice],
            SkillType::PiercingStrike => {
                vec![SkillAttribute::ArmorPiercing, SkillAttribute::Physical]
            }
            SkillType::LanceOfLight => vec![SkillAttribute::Physical, SkillAttribute::Light],
            SkillType::Shield => vec![SkillAttribute::Defensive],
            SkillType::DarkSpear => vec![SkillAttribute::Physical, SkillAttribute::Dark],
            SkillType::DeathFromAbove => {
                vec![SkillAttribute::Physical, SkillAttribute::AreaOfEffect]
            }
        };
    }
}

impl PromptOption for Skill {
    fn option_name(&self) -> String {
        return String::from(match self.skill_type {
            SkillType::FireMagic => "Fire Magic",
            SkillType::WaterMagic => "Water Magic",
            SkillType::LightningMagic => "Lightning Magic",
            SkillType::IceMagic => "Ice Magic",
            SkillType::ExplosionMagic => "Explosion Magic",
            SkillType::WindMagic => "Wind Magic",
            SkillType::DarkMagic => "Dark Magic",
            SkillType::UnholyCurse => "Unholy Curse",
            SkillType::LightMagic => "Light Magic",
            SkillType::DivineBlessing => "Divine Blessing",
            SkillType::WeatherForecast => "Weather Forecast",
            SkillType::SwordStrike => "Sword Strike",
            SkillType::DuelStrike => "Duel Strike",
            SkillType::DefensiveForm => "Defensive Form",
            SkillType::SwordSummoning => "Sword Summoning",
            SkillType::TempestStrike => "Tempest Strike",
            SkillType::FlamingStrike => "Flaming Strike",
            SkillType::FrozenStrike => "Frozen Strike",
            SkillType::PiercingStrike => "Piercing Strike",
            SkillType::LanceOfLight => "Lance of Light",
            SkillType::Shield => "Shield",
            SkillType::DarkSpear => "Dark Spear",
            SkillType::DeathFromAbove => "Death from Above",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        return None;
    }
}

/// Attributes describe a Skill and are used to determine effectiveness in battle
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SkillAttribute {
    Magic,
    Physical,
    Fire,
    Ice,
    Electric,
    Water,
    Wind,
    Dark,
    Light,
    Healing,
    Defensive,
    MagicResistive,
    DamageOverTime,
    AreaOfEffect,
    ArmorPiercing,
    MultiHit(u16),
}

impl PromptOption for SkillAttribute {
    fn option_name(&self) -> String {
        return String::from(match self {
            SkillAttribute::Magic => "Magic",
            SkillAttribute::Physical => "Physical",
            SkillAttribute::Fire => "Fire",
            SkillAttribute::Ice => "Ice",
            SkillAttribute::Electric => "Electric",
            SkillAttribute::Water => "Water",
            SkillAttribute::Wind => "Wind",
            SkillAttribute::Dark => "Dark",
            SkillAttribute::Light => "Light",
            SkillAttribute::Healing => "Healing",
            SkillAttribute::Defensive => "Defensive",
            SkillAttribute::MagicResistive => "Magic Resistive",
            SkillAttribute::DamageOverTime => "Damage Over Time",
            SkillAttribute::AreaOfEffect => "Area of Effect",
            SkillAttribute::ArmorPiercing => "Armor Piercing",
            SkillAttribute::MultiHit(_) => "Multi-Hit",
        });
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
