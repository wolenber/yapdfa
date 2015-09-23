//! Yet Another PDF Attempt

#![deny(missing_docs)]

#[macro_use] mod indirect;

mod catalog;
mod document;
mod objects;
mod output;
mod page;
mod pages;
mod rectangle;
mod trailer;
mod version;
mod xref;

pub use document::Document;
pub use page::Page;
pub use pages::Pages;
pub use version::Version;