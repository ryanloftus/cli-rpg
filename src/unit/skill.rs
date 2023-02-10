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
    CrazyMagic,
    WeatherForecast,
    StormSummoning,
    SwordDance,
    QuickDraw,
    DuelSlash,
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
        todo!()
    }
}

impl PromptOption for Skill {
    fn option_name(&self) -> String {
        todo!()
    }

    fn short_option_name(&self) -> Option<String> {
        None
    }
}
