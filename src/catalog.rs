use prelude::*;
use pages::Pages;
use rectangle::LETTER_PAGE;

/// A dictionary, and the root of the PDF object tree.
#[derive(Debug)]
pub struct Catalog {
    self_reference: IndirectFields,
    pages: PdfRef<Pages>,
}

impl Catalog {
    /// Creates a new Catalog, with a new Pages child
    pub fn new() -> Catalog {
        let mut pages = Pages::new();
        pages.set_media_box(Some(LETTER_PAGE));
        Catalog::new_with_pages(pages)
    }

    /// Creates a new Catalog from an existing Pages object
    pub fn new_with_pages(pages: Pages) -> Catalog {
        Catalog {
            self_reference: IndirectFields::new(),
            pages: pdf_ref_new(pages)
        }
    }

    /// Returns the Pages child of this Catalog
    pub fn get_pages_mut(&mut self) -> PdfRef<Pages> {
        self.pages.clone()
    }
}

indirect_object!(for Catalog as self_reference);

impl ToOutput for Catalog {
    fn to_output(&self) -> Box<Output> {
        let mut dict = Dictionary::new();
        dict.set("Type", Name::new("Catalog"));
        dict.set("Pages", Reference::from_ref(&self.pages));
        Box::new(dict)
    }
}