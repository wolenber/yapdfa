/// PDF version
#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
#[derive(Hash)]
pub enum Version {
    /// 1.0
    V10,
    /// 1.1
    V11,
    /// 1.2
    V12,
    /// 1.3
    V13,
    /// 1.4
    V14,
    /// 1.5
    V15,
    /// 1.6
    V16,
    /// 1.7
    V17,
}

impl Version {
    /// Returns the version string of a PDF file, not including newline.
    /// Example output: %PDF-1.0
    pub fn version_string(self) -> &'static str {
        use Version::*;
        match self {
            V10 => "%PDF-1.0",
            V11 => "%PDF-1.1",
            V12 => "%PDF-1.2",
            V13 => "%PDF-1.3",
            V14 => "%PDF-1.4",
            V15 => "%PDF-1.5",
            V16 => "%PDF-1.6",
            V17 => "%PDF-1.7",
        }
    }
}