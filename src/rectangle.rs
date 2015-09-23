/// A rectangle, with lower left corner at (x, y),
/// extending height upwards and width to the right.
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Default)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
#[derive(Ord, PartialOrd)]
pub struct Rectangle {
    /// Left edge
    pub x: i64,
    /// Bottom edge
    pub y: i64,
    /// Width
    pub width: i64,
    /// Height
    pub height: i64,
}

/// Rectangle with user-space dimensions of an 8.5in by 11in page
pub const LETTER_PAGE: Rectangle =
    Rectangle { x: 0, y: 0, width: 612, height: 792 };