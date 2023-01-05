pub mod attribute;
use attribute::SkillAttribute;

#[derive(Debug, Clone)]
pub struct Skill {
    name: &'static str,
    unique_id: &'static str,
    attributes: &'static [SkillAttribute],
}

impl Skill {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }

    const fn new_const(name: &'static str, unique_id: &'static str, attributes: &'static [SkillAttribute]) -> Self {
        Self { name, unique_id, attributes }
    }
}
