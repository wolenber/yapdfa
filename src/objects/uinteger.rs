use prelude::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Ord, PartialOrd)]
pub struct UInteger(pub u64);

impl Output for UInteger {
    fn output(&self) -> String {
        format!("{}", self.0)
    }
}