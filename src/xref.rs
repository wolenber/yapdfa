use prelude::*;
use std::collections::BTreeMap;
use std::u16;

/// A cross-reference table permitting random-access into indirect objects
/// Values are a tuple of (byte position, generation number, object status)
#[derive(Debug)]
#[derive(Default)]
pub struct Xref(BTreeMap<u64, (u64, u16, ObjectStatus)>);

#[derive(Copy, Clone)]
#[derive(Debug)]
#[derive(Eq, PartialEq)]
enum ObjectStatus {
    InUse,
    Free,
}

impl Xref {
    pub fn new() -> Xref {
        let mut xref = Xref(BTreeMap::new());
        xref.0.insert(0, (0, u16::MAX, ObjectStatus::Free));
        xref
    }

    pub fn insert(&mut self, id: u64, byte_pos: u64) {
        self.0.insert(id, (byte_pos, 0, ObjectStatus::InUse));
    }

    pub fn get_size(&self) -> u64 {
        self.0.len() as u64
    }
}

impl Output for Xref {
    fn output(&self) -> String {
        let key_groups = {
            // Note some tricky things:
            // - the first group is initialized with a 0 key
            // - we skip the first key, which ought to be 0 with unsigned keys
            // This avoid a subtle off-by-one/overflow bug.
            let mut groups = Vec::new();
            let mut group = vec![0];
            let mut last_key = 0;
            for key in self.0.keys().skip(1) {
                if (key-1) != last_key {
                    groups.push(group);
                    group = Vec::new();
                }
                group.push(*key);
                last_key = *key;
            }
            if group.len() > 0 {
                groups.push(group);
            }
            groups
        };
        let mut buf = String::new();
        buf = buf + "xref" + "\n";
        for group in key_groups {
            let first_key = group[0];
            let num_keys = group.len();
            buf = buf + &format!("{} {}", first_key, num_keys) + "\n";
            for key in group {
                let (byte_pos, gen_num, status) = self.0[&key];
                let status = if status == ObjectStatus::InUse { 'n' } else { 'f' };
                buf = buf + &format!("{:0>10} {:0>5} {} \n", byte_pos, gen_num, status);
            }
        }
        buf
    }
}