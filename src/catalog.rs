use std::cell::RefCell;
use std::rc::Rc;
use indirect::*;
use objects::*;
use output::*;
use pages::Pages;

/// A dictionary, and the root of the PDF object tree.
#[derive(Debug)]
pub struct Catalog {
    self_reference: IndirectFields,
    pages: Rc<RefCell<Pages>>,
}

impl Catalog {
    /// Creates a new Catalog, with a new Pages child
    pub fn new() -> Catalog {
        Catalog::new_with_pages(Pages::new())
    }

    /// Creates a new Catalog from an existing Pages object
    pub fn new_with_pages(pages: Pages) -> Catalog {
        Catalog {
            self_reference: IndirectFields::new(),
            pages: Rc::new(RefCell::new(pages))
        }
    }

    /// Returns the Pages child of this Catalog
    pub fn get_pages_mut(&mut self) -> Rc<RefCell<Pages>> {
        self.pages.clone()
    }
}

indirect_object!(for Catalog as self_reference);

impl ToOutput for Catalog {
    fn to_output(&self) -> Box<Output> {
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Catalog"));
        dict.set("Pages", Reference::from(&*self.pages.borrow()));
        Box::new(dict)
    }
}