use indirect::IndirectObject;
use output::Output;

#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Reference(u64);

impl Reference {
    pub fn from<T: IndirectObject>(from: &T) -> Reference {
        Reference(from.get_id().unwrap())
    }
    
    pub fn from_u64(from: u64) -> Reference {
        Reference(from)
    }
}

impl Output for Reference {
    fn output(&self) -> String {
        format!("{} 0 R", self.0)
    }
}