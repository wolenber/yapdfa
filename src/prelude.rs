use std::cell::RefCell;
use std::rc::Rc;
use std::rc::Weak;

pub type PdfRef<T> = Rc<RefCell<T>>;
pub type WeakPdfRef<T> = Weak<RefCell<T>>;

pub fn pdf_ref_new<T>(val: T) -> PdfRef<T> { Rc::new(RefCell::new(val)) }

pub use indirect::IndirectFields;
pub use indirect::IndirectObject;
pub use objects::Array;
pub use objects::Dictionary;
pub use objects::Integer;
pub use objects::Name;
pub use objects::Reference;
pub use objects::UInteger;
pub use output::Output;
pub use output::ToOutput;
pub use rectangle::Rectangle;