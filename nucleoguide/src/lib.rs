mod algorithm;

pub struct GuideDesign {
    seq: String
}

impl GuideDesign {
    pub fn new(s: String) -> Self {
        GuideDesign { seq: s }
    }
    pub fn design(&self) -> &str {
        todo!();
    }
}

