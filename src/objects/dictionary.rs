use objects::Name;
use output::Output;

#[derive(Debug)]
pub struct Dictionary(Vec<(Name, Box<Output>)>);

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary(Vec::new())
    }

    pub fn set<S: Into<String>, T: Output + 'static>(&mut self, key: S, value: T) {
        self.0.push((Name::new(key), Box::new(value)));
    }
}

impl Output for Dictionary {
    fn output(&self) -> String {
        let lines = {
            let mut buf = Vec::new();
            buf.push("<<".to_owned());
            for entry in self.0.iter() {
                buf.push(format!("{} {}", entry.0.output(), entry.1.output()));
            }
            buf.push(">>".to_owned());
            buf
        };
        lines.join("\n")
    }
}