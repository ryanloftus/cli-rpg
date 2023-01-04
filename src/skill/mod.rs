pub mod skill_attributes;
use std::borrow::Cow;
use skill_attributes::SkillAttribute;

#[derive(Debug, Clone)]
pub struct Skill {
    name: Cow<'static, str>,
    unique_id: Cow<'static, str>,
    attributes: Vec<SkillAttribute>,
}

impl Skill {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }

    const fn new_const(name: &'static str, unique_id: &'static str, attributes: &'static [SkillAttribute]) -> Self {
        Self {
            name: Cow::Borrowed(name),
            unique_id: Cow::Borrowed(unique_id),
            attributes: attributes.to_vec(),
        }
    }
}
