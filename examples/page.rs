extern crate yapdfa;

use yapdfa::Document;

fn main() {
    let mut doc = Document::new();
    doc.push_page();
    let out = doc.output();
    println!("{}", out);
}