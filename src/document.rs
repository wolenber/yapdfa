use prelude::*;
use catalog::Catalog;
use page::Page;
use pages::Pages;
use trailer::Trailer;
use version::Version;
use xref::Xref;

/// A PDF document, the contents of a PDF file.
#[derive(Debug)]
pub struct Document {
    catalog: PdfRef<Catalog>,
    version: Version,
    pages_list: Vec<PdfRef<Pages>>,
    page_list: Vec<PdfRef<Page>>,
}

impl Document {
    /// Creates and returns a new document
    pub fn new() -> Document {
        let mut doc = Document {
            catalog: pdf_ref_new(Catalog::new()),
            version: Version::V10,
            pages_list: Vec::new(),
            page_list: Vec::new(),
        };
        doc.pages_list.push(doc.catalog.borrow_mut().get_pages_mut());
        doc
    }

    /// Appends a page to the end of the document
    pub fn push_page(&mut self) -> PdfRef<Page> {
        let pages = self.catalog.borrow_mut().get_pages_mut();
        let page = Page::with_parent(PdfRef::downgrade(&pages));
        let page = pdf_ref_new(page);
        pages.borrow_mut().push_page(page.clone());
        self.page_list.push(page.clone());
        page
    }

    /// Write the pdf to a byte vector
    pub fn output(&mut self) -> String {
        // Create an iterator over all objects
        let mut objs: Vec<PdfRef<IndirectObject>> = Vec::new();
        objs.push(self.catalog.clone());
        objs.extend(self.pages_list.iter().map(|pages| pages.clone() as PdfRef<IndirectObject>));
        objs.extend(self.page_list.iter().map(|page| page.clone() as PdfRef<IndirectObject>));
        let objs = &objs;

        // Set all object IDs
        let mut id = 1;
        for obj in objs {
            let mut obj = obj.borrow_mut();
            obj.set_id(id);
            id += 1;
        }

        // Write the file
        let mut buf = String::new();
        let mut xref = Xref::new();
        let mut trailer = Trailer::new();
        buf = buf + self.version.version_string() + "\n\n";
        for obj in objs {
            let mut obj = obj.borrow_mut();
            let byte_pos = buf.len() as u64;
            obj.set_byte_pos(byte_pos);
            xref.insert(obj.get_id().unwrap(), byte_pos);
            buf = buf + &obj.output() + "\n\n";
        }
        trailer.set_xref_pos(buf.len() as u64);
        trailer.set_root(self.catalog.borrow().get_id().unwrap());
        trailer.set_size(xref.get_size());
        buf = buf + &xref.output() + "\n";
        buf = buf + &trailer.output();
        let output = buf;

        // Clear the IndirectObject state
        for obj in objs {
            obj.borrow_mut().clear();
        }
        
        // return
        output
    }
}