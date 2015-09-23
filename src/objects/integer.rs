use output::Output;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Integer(i64);

impl Integer {
    pub fn new(s: i64) -> Integer {
        Integer(s)
    }
}

impl Output for Integer {
    fn output(&self) -> String {
        format!("{}", self.0)
    }
}