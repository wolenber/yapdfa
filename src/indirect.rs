use std::fmt::Debug;
use output::*;

/// Struct containing indirect fields of an indirect object.
/// Allows making it prettier, I hope.
#[derive(Debug)]
pub struct IndirectFields {
    pub id: Option<u64>,
    pub byte_pos: Option<u64>,
}

impl IndirectFields {
    pub fn new() -> IndirectFields {
        IndirectFields {
            id: None,
            byte_pos: None,
        }
    }
}

impl Output for IndirectFields {
    fn output(&self) -> String {
        unreachable!("If this code is called, there is a library error.")
    }
}

impl IndirectObject for IndirectFields {
    fn get_id(&self) -> Option<u64> { self.id }
    fn set_id(&mut self, val: u64) { self.id = Some(val); }
    fn clear_id(&mut self) { self.id = None; }
    fn get_byte_pos(&self) -> Option<u64> { self.byte_pos }
    fn set_byte_pos(&mut self, val: u64) { self.byte_pos = Some(val); }
    fn clear_byte_pos(&mut self) { self.byte_pos = None; }
}

/// Getters and setters so indirect objects can use a common interface.
pub trait IndirectObject : Output {
    /// Set the object to have neither an ID nor a byte position.
    /// This is useful for after creating the PDF, to make sure state is reset to something sane.
    fn clear(&mut self) {
        self.clear_id();
        self.clear_byte_pos();
    }

    /// Gets the ID, if any.
    fn get_id(&self) -> Option<u64>;
    /// Sets the ID.
    fn set_id(&mut self, u64);
    /// Clears the ID.
    fn clear_id(&mut self);

    /// Gets the byte position, if any.
    fn get_byte_pos(&self) -> Option<u64>;
    /// Sets the byte position.
    fn set_byte_pos(&mut self, u64);
    /// Clears the byte position.
    fn clear_byte_pos(&mut self);
}

impl <T> Output for T where T: ToOutput + IndirectObject + Debug {
    fn output(&self) -> String {
        let output = self.to_output();
        format!("{} 0 obj\n{}\nendobj", self.get_id().unwrap(), output.output())
    }
}

macro_rules! indirect_object {
    (for $obj:ty as $rf:ident) => {
        impl IndirectObject for $obj {
            fn get_id(&self) -> Option<u64> { self.$rf.get_id() }
            fn set_id(&mut self, val: u64) { self.$rf.set_id(val); }
            fn clear_id(&mut self) { self.$rf.clear_id(); }
            fn get_byte_pos(&self) -> Option<u64> { self.$rf.get_byte_pos() }
            fn set_byte_pos(&mut self, val: u64) { self.$rf.set_byte_pos(val); }
            fn clear_byte_pos(&mut self) { self.$rf.clear_byte_pos(); }
        }
    }
}