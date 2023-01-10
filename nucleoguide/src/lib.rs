use bio::alphabets::{self};

mod grnas;

pub struct GuideDesign {
    seq: String,
    pam: String,
    pam_offset: usize,
}

impl GuideDesign {
    pub fn new(s: String, p: String, po: usize) -> Self {
        let dna_ab = alphabets::dna::alphabet();
        if dna_ab.is_word(s.as_bytes()) == false {
            panic!("Provided DNA String is not valid.")
        } else {
            GuideDesign {
                seq: s,
                pam: p, 
                pam_offset: po
            }
        }
    }
    pub fn run(&self) -> &str {
        todo!();
    }
}

