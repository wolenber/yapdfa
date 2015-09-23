use prelude::*;
use page::Page;

/// A tree of pages, containing both Page and Pages objects
#[derive(Debug)]
pub struct Pages {
    self_reference: IndirectFields,
    parent: Option<WeakPdfRef<Pages>>,
    kids: Vec<Leaf>,
    media_box: Option<Rectangle>,
}

/// A reference-counted ref-cell to either a Page or a Pages
#[derive(Debug)]
enum Leaf {
    Pages(PdfRef<Pages>),
    Page(PdfRef<Page>),
}

impl Pages {
    /// Creates and returns a new Pages, with no parent or kids
    pub fn new() -> Pages {
        Pages {
            self_reference: IndirectFields::new(),
            parent: None,
            media_box: None,
            kids: Vec::new(),
        }
    }

    /// Push a kid onto the current pages object
    pub fn push_page(&mut self, page: PdfRef<Page>) {
        self.kids.push(Leaf::Page(page));
    }

    /// Push a kid onto the current pages object
    pub fn push_pages(&mut self, pages: PdfRef<Pages>) {
        self.kids.push(Leaf::Pages(pages));
    }

    /// Sets the Media Box of this Pages object
    pub fn set_media_box(&mut self, mbox: Option<Rectangle>) {
        self.media_box = mbox;
    }
}

indirect_object!(for Pages as self_reference);

impl ToOutput for Pages {
    fn to_output(&self) -> Box<Output> {
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Pages"));
        dict.set("Count", Integer(self.kids.len() as i64));
        if self.kids.len() > 0 {
            let mut array = Array::new();
            for kid in self.kids.iter() {
                match kid {
                    &Leaf::Pages(ref a) => array.push(Reference::from_ref(a)),
                    &Leaf::Page(ref a) => array.push(Reference::from_ref(a)),
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