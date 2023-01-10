mod algorithm;

pub struct Sequence {
    seq: String
}

impl Sequence {
    pub fn new(s: String) -> Self {
        Sequence { seq: s }
    }
    pub fn design(&self) -> &str {
        todo!();
    }
}

