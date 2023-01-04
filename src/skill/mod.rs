use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Skill {
    name: Cow<'static, str>,
    unique_id: Cow<'static, str>,
}

impl Skill {
    pub fn to_option_string(&self) -> String {
        format!("({}) {}", self.unique_id, self.name)
    }

    const fn new_const(name: &'static str, unique_id: &'static str) -> Self {
        Self {
            name: Cow::Borrowed(name),
            unique_id: Cow::Borrowed(unique_id),
        }
    }
}
