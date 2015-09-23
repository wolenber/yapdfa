use objects::Integer;
use output::Output;
use rectangle::Rectangle;

#[derive(Debug)]
pub struct Array(Vec<Box<Output>>);

impl Array {
    pub fn new() -> Array {
        Array(Vec::new())
    }

    pub fn from_rect(r : Rectangle) -> Array {
        let mut buf = Array::new();
        buf.push(Integer::new(r.x));
        buf.push(Integer::new(r.y));
        buf.push(Integer::new(r.width));
        buf.push(Integer::new(r.height));
        buf 
    }

    pub fn push<T: Output + 'static>(&mut self, value: T) {
        self.0.push(Box::new(value));
    }
}

impl Output for Array {
    fn output(&self) -> String {
        /// Chosen so [ x y width height ] always stays one line.
        const LINES_BREAK_AFTER: usize = 6;

        let lines = {
            let mut buf = Vec::new();
            buf.push("[".to_owned());
            buf.extend(self.0.iter().map(|e| e.output()));
            buf.push("]".to_owned());
            buf
        };

        lines.join(if lines.len() > LINES_BREAK_AFTER { "\n" } else { " " })
    }
}