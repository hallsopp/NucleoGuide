use std::collections::HashSet;
use bio::alphabets;
use crate::errors::errors::RuntimeError;
use crate::grnas::grnas::run;
mod grnas;
mod errors;

#[derive(Debug)]
pub struct GuideDesign {
    seq: String,
    pam: String,
    guide_features: Grna
}

#[derive(Debug)]
struct Grna {
    size: usize,
    xc_pattern: String
}

impl GuideDesign {
    // instantise a new guide design object 
    pub fn new(s: String, p: String, gf_size: usize, gf_xc_pattern: String) -> Result<Self, RuntimeError> {
        let dna_ab = alphabets::dna::alphabet();
        // check that input seqeunce is valid DNA
        if dna_ab.is_word(s.as_bytes()) == false {
            Err(RuntimeError::IncorrectDNASequence)
        } else {
            Ok(GuideDesign {
                seq: s,
                pam: p,
                guide_features: Grna { size: gf_size, xc_pattern: gf_xc_pattern }
            })
        }
    }
    // Funtion to search for grnas, at the moment returns the first instance
    // TODO: return list of grnas in a certain format that is currently not decided
    // on :D 
    pub fn idgrnas(&self) -> Result<HashSet<&str>, RuntimeError> {
        match run(&self.seq, &self.pam, &self.guide_features.size, &self.guide_features.xc_pattern) {
            Ok(n) => Ok(n),
            Err(n) => Err(n)
        } 
    }
}


