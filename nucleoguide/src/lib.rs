use bio::alphabets::{self};

mod algorithm;

pub struct GuideDesign {
    seq: String
}

impl GuideDesign {
    pub fn new(s: String) -> Self {
        let dna_ab = alphabets::dna::alphabet();
        assert!(dna_ab.is_word(s.clone().into_bytes()));
        GuideDesign { seq: s }
    }
    pub fn run(&self) -> &str {
        todo!();
    }
}

