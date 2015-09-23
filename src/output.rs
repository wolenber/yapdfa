use std::fmt::Debug;

/// Output types can be written to a PDF
pub trait Output : Debug {
    /// Return a string represting the object in a PDF
    fn output(&self) -> String;
}

/// Trait for internal types to indicate they can be converted to PDF types
/// FIXME: This out to use associated types
pub trait ToOutput {
    /// Create an Output internally, then return it.
    fn to_output(&self) -> Box<Output>;
}