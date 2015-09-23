use std::cell::RefCell;
use std::rc::Weak;
use indirect::*;
use objects::*;
use output::*;
use pages::Pages;
use rectangle::Rectangle;

/// A single page containing text, graphics, and/or images.
#[derive(Debug)]
pub struct Page {
    self_reference: IndirectFields,
    parent: Weak<RefCell<Pages>>,
    media_box: Option<Rectangle>,
}

impl Page {
    /// Create a new Page object, with a parent
    pub fn new(parent: Weak<RefCell<Pages>>) -> Page {
        Page {
            self_reference: IndirectFields::new(),
            parent: parent,
            media_box: None,
        }
    }
}

indirect_object!(for Page as self_reference);

impl ToOutput for Page {
    fn to_output(&self) -> Box<Output> {
        let parent = self.parent.upgrade().unwrap();
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Page"));
        dict.set("Parent", Reference::from(&*parent.borrow()));
        Box::new(dict)
    }
}