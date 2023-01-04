use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct SkillAttribute {
    name: Cow<'static, str>,
}

impl SkillAttribute {
    const fn new_const(name: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
        }
    }
}

mod attributes {
    use super::SkillAttribute;

    pub const RANGED: SkillAttribute = SkillAttribute::new_const("Ranged");
    pub const MELEE: SkillAttribute = SkillAttribute::new_const("Melee");
    pub const MAGIC: SkillAttribute = SkillAttribute::new_const("Magic");
    // TODO: add more skill attributes
}
