use enum_dispatch::enum_dispatch;

#[enum_dispatch]
pub trait Test {
    fn matches(&self) -> bool;
}

#[enum_dispatch(Test)]
pub enum Condition {
    And,
    Or,
}

pub struct And(Vec<Condition>);

impl Test for And {
    fn matches(&self) -> bool {
        self.0.iter().all(Test::matches)
    }
}

pub struct Or(Vec<Condition>);

impl Test for Or {
    fn matches(&self) -> bool {
        self.0.iter().any(Test::matches)
    }
}

pub struct HasLabels(Vec<String>);
