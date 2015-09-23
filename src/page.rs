use prelude::*;
use pages::Pages;

/// A single page containing text, graphics, and/or images.
#[derive(Debug)]
pub struct Page {
    self_reference: IndirectFields,
    parent: WeakPdfRef<Pages>,
    media_box: Option<Rectangle>,
}

impl Page {
    /// Create a new Page object, with a parent
    pub fn with_parent(parent: WeakPdfRef<Pages>) -> Page {
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
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Page"));
        dict.set("Parent", Reference::from_ref(&self.parent.upgrade().unwrap()));
        Box::new(dict)
    }
}