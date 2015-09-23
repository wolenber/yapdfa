use prelude::*;

/// A hook to allow easily finding the xrefs table and catalog
#[derive(Debug)]
#[derive(Default)]
pub struct Trailer {
    xref_pos: u64,
    size: u64,
    root: u64,
}

/*
trailer
<<
/Size 22
/Root 2 0 R
/Info 1 0 R
>>
startxref
18799
%%EOF
*/

impl Trailer {
    pub fn new() -> Trailer {
        Trailer {
            xref_pos: 0,
            size: 0,
            root: 0,
        }
    }

    pub fn set_xref_pos(&mut self, pos: u64) {
        self.xref_pos = pos;
    }
    
    pub fn set_size(&mut self, size: u64) {
        self.size = size;
    }
    
    pub fn set_root(&mut self, root: u64) {
        self.root = root;
    }
}

impl Output for Trailer {
    fn output(&self) -> String {
        let mut dict = Dictionary::new();
        dict.set("Size", UInteger(self.size));
        dict.set("Root", Reference::from_u64(self.root));
        let dict = dict.output();
        let mut buf = String::new();
        buf = buf + "trailer" + "\n";
        buf = buf + &dict + "\n";
        buf = buf + "startxref" + "\n";
        buf = buf + &format!("{}", self.xref_pos) + "\n";
        buf = buf + "%%EOF";
        buf
    }
}