use prelude::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Ord, PartialOrd)]
pub struct Integer(pub i64);

impl Output for Integer {
    fn output(&self) -> String {
        format!("{}", self.0)
    }
}