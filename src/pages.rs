use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;
use indirect::*;
use objects::*;
use output::*;
use page::Page;
use rectangle::Rectangle;

/// A tree of pages, containing both Page and Pages objects
#[derive(Debug)]
pub struct Pages {
    self_reference: IndirectFields,
    parent: Option<Weak<RefCell<Pages>>>,
    kids: Vec<Leaf>,
    media_box: Option<Rectangle>,
}

/// A reference-counted ref-cell to either a Page or a Pages
#[derive(Debug)]
enum Leaf {
    Pages(Rc<RefCell<Pages>>),
    Page(Rc<RefCell<Page>>),
}

impl Pages {
    /// Creates and returns a new Pages, with no parent or kids
    pub fn new() -> Pages {
        Pages {
            self_reference: IndirectFields::new(),
            parent: None,
            media_box: Some(Rectangle { x: 0, y: 0, width: 612, height: 792 }),
            kids: Vec::new(),
        }
    }

    /// Push a kid onto the current pages object
    pub fn push_page(&mut self, page: Rc<RefCell<Page>>) {
        self.kids.push(Leaf::Page(page));
    }
}

indirect_object!(for Pages as self_reference);

impl ToOutput for Pages {
    fn to_output(&self) -> Box<Output> {
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Pages"));
        dict.set("Count", Integer::new(self.kids.len() as i64));
        if self.kids.len() > 0 {
            let mut array = Array::new();
            for kid in self.kids.iter() {
                match kid {
                    &Leaf::Pages(ref a) => array.push(Reference::from(&*a.borrow())),
                    &Leaf::Page(ref a) => array.push(Reference::from(&*a.borrow())),
                }
            }
            dict.set("Kids", array);
        }
        if let Some(r) = self.media_box {
            dict.set("MediaBox", Array::from_rect(r));
        }
        Box::new(dict)
    }
}