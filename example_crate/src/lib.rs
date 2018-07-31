use std::fmt;

#[derive(Debug)]
pub struct Struct {
    string: &'static str,
    ch: char,
}

impl Struct {
    pub fn new() -> Self {
        Struct {
            string: "'ello!",
            ch: 'H',
        }
    }
}

impl fmt::Display for Struct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.string.replace("'", &self.ch.to_string()))
    }
}