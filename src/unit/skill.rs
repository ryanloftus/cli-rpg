use super::attribute::Attribute;
use super::experience::Experience;
use crate::prompt::PromptOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub skill_type: SkillType,
    pub experience: Experience,
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
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
}

impl SkillType {
    fn base_attributes(&self) -> Vec<Attribute> {
        return match self {
            SkillType::FireMagic => vec![Attribute::Magic, Attribute::Fire],
            SkillType::WaterMagic => vec![Attribute::Magic, Attribute::Water],
            SkillType::LightningMagic => vec![Attribute::Magic, Attribute::Electric],
            SkillType::IceMagic => vec![Attribute::Magic, Attribute::Ice],
            SkillType::ExplosionMagic => {
                vec![Attribute::Magic, Attribute::Fire, Attribute::AreaOfEffect]
            }
            SkillType::WindMagic => vec![Attribute::Magic, Attribute::Wind],
            SkillType::DarkMagic => vec![Attribute::Magic, Attribute::Dark],
            SkillType::UnholyCurse => {
                vec![Attribute::Magic, Attribute::Dark, Attribute::DamageOverTime]
            }
            SkillType::LightMagic => vec![Attribute::Magic, Attribute::Light, Attribute::Healing],
            SkillType::DivineBlessing => vec![
                Attribute::Magic,
                Attribute::Light,
                Attribute::Healing,
                Attribute::MagicResistive,
            ],
            SkillType::WeatherForecast => vec![
                Attribute::Magic,
                Attribute::Water,
                Attribute::Fire,
                Attribute::Wind,
                Attribute::Electric,
                Attribute::Ice,
                Attribute::AreaOfEffect,
            ],
            SkillType::SwordStrike => vec![Attribute::Physical],
            SkillType::DuelStrike => vec![Attribute::Physical, Attribute::MultiHit(2)],
            SkillType::DefensiveForm => vec![Attribute::Defensive],
            SkillType::SwordSummoning => vec![
                Attribute::Physical,
                Attribute::Magic,
                Attribute::AreaOfEffect,
                Attribute::MultiHit(5),
            ],
            SkillType::TempestStrike => {
                vec![Attribute::Physical, Attribute::Magic, Attribute::Wind]
            }
            SkillType::FlamingStrike => {
                vec![Attribute::Physical, Attribute::Magic, Attribute::Fire]
            }
            SkillType::FrozenStrike => vec![Attribute::Physical, Attribute::Magic, Attribute::Ice],
            SkillType::PiercingStrike => vec![Attribute::ArmorPiercing, Attribute::Physical],
            SkillType::LanceOfLight => {
                vec![Attribute::Physical, Attribute::Magic, Attribute::Light]
            }
            SkillType::Shield => vec![Attribute::Defensive],
            SkillType::DarkSpear => vec![Attribute::Magic, Attribute::Physical, Attribute::Dark],
            SkillType::DeathFromAbove => vec![
                Attribute::Magic,
                Attribute::Physical,
                Attribute::AreaOfEffect,
            ],
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
