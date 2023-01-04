use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Area {
    pub name: Cow<'static, str>,
    pub unique_id: Cow<'static, str>,
    // TODO: add progress
}

impl Area {
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
