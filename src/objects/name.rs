use prelude::*;

#[derive(Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Ord, PartialOrd)]
pub struct Name(pub String);

impl Name {
    pub fn new<T: Into<String>>(s: T) -> Name {
        Name(s.into())
    }
}

impl Output for Name {
    fn output(&self) -> String {
        format!("/{}", self.0)
    }
}