use output::Output;

#[derive(Clone)]
#[derive(Debug)]
pub struct Name(String);

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