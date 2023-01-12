use bio::alphabets;
use crate::errors::errors::RuntimeError;
use crate::grnas::grnas::find;
mod grnas;
mod errors;

pub struct GuideDesign {
    seq: String,
    pam: String,
    pam_offset: usize,
}

impl GuideDesign {
    // instantise a new guide design object 
    pub fn new(s: String, p: String, po: usize) -> Result<Self, RuntimeError> {
        let dna_ab = alphabets::dna::alphabet();
        // check that input seqeunce is valid DNA
        // TODO: move this to a seperate function to make this a bit 
        // cleaner
        if dna_ab.is_word(s.as_bytes()) == false {
            Err(RuntimeError::IncorrectDNASequence)
        } else {
            Ok(GuideDesign {
                seq: s,
                pam: p, 
                pam_offset: po
            })
        }
    }
    // Funtion to search for grnas, at the moment returns the first instance
    // TODO: return list of grnas in a certain format that is currently not decided
    // on :D 
    pub fn idgrnas(&self) -> Result<usize, RuntimeError> {
        match find(&self.seq, &self.pam, &self.pam_offset) {
            Ok(n) => Ok(n),
            Err(n) => Err(n)
        } 
    }
}

