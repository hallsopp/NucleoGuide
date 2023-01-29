use crate::errors::RuntimeError;
use crate::grnas::run;
use bio::alphabets;
mod errors;
mod grnas;

#[derive(Debug)]
pub struct GuideDesign {
    seq: String,
    pam: String,
    guide_features: Grna,
}

#[derive(Debug)]
struct Grna {
    size: usize,
    xc_pattern: String,
    ic_pattern: String,
}

impl GuideDesign {
    // instantise a new guide design object
    pub fn new(
        s: String,
        p: String,
        gf_size: usize,
        gf_xc_pattern: String,
        gf_ic_pattern: String,
    ) -> Result<Self, RuntimeError> {
        let dna_ab = alphabets::dna::alphabet();
        // check that input seqeunce is valid DNA
        if !dna_ab.is_word(s.as_bytes()) {
            Err(RuntimeError::IncorrectDNASequence)
        } else {
            Ok(GuideDesign {
                seq: s,
                pam: p,
                guide_features: Grna {
                    size: gf_size,
                    xc_pattern: gf_xc_pattern,
                    ic_pattern: gf_ic_pattern,
                },
            })
        }
    }
    // Funtion to search for grnas
    pub fn idgrnas(&self) -> Result<Vec<(&str, usize)>, RuntimeError> {
        match run(
            &self.seq,
            &self.pam,
            &self.guide_features.size,
            &self.guide_features.xc_pattern,
            &self.guide_features.ic_pattern,
        ) {
            Ok(n) => Ok(n),
            Err(n) => Err(n),
        }
    }
}
