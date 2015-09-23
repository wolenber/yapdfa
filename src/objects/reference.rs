use prelude::*;

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Ord, PartialOrd)]
pub struct Reference(u64);

impl Reference {
    pub fn from_ref<T: IndirectObject>(from: &PdfRef<T>) -> Reference {
        Reference(from.borrow().get_id().unwrap())
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