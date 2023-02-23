use crate::errors::RuntimeError;
use crate::grnas::run;
use bio::alphabets;
use std::collections::HashMap;
use std::str::from_utf8;
mod errors;
mod grnas;
mod offtarget;

#[derive(Debug)]
pub struct GuideDesign {
    seq: String,
    revcomp: String,
    pam: String,
    guide_features: Grna,
}

#[derive(Debug)]
struct Grna {
    size: usize,
    xc_pattern: String,
    ic_pattern: String,
    min_gc: f32,
    max_gc: f32,
}

impl GuideDesign {
    // instantise a new guide design object
    pub fn new(
        s: String,
        p: String,
        gf_size: usize,
        gf_xc_pattern: String,
        gf_ic_pattern: String,
        gf_min_gc: f32,
        gf_max_gc: f32,
    ) -> Result<Self, RuntimeError> {
        let check = s.as_bytes();
        let dna_ab = alphabets::dna::alphabet();
        // check that input seqeunce is valid DNA
        if !dna_ab.is_word(check) {
            Err(RuntimeError::IncorrectDNASequence)
        } else {
            let rc = get_revcomp(check)?;
            Ok(GuideDesign {
                seq: s,
                revcomp: rc,
                pam: p,
                guide_features: Grna {
                    size: gf_size,
                    xc_pattern: gf_xc_pattern,
                    ic_pattern: gf_ic_pattern,
                    min_gc: gf_min_gc,
                    max_gc: gf_max_gc,
                },
            })
        }
    }
    // Funtion to search for grnas
    pub fn idgrnas(&self) -> Result<HashMap<String, Vec<(&str, usize)>>, RuntimeError> {
        match run(
            &self.seq,
            &self.revcomp,
            &self.pam,
            &self.guide_features.size,
            &self.guide_features.xc_pattern,
            &self.guide_features.ic_pattern,
            &self.guide_features.min_gc,
            &self.guide_features.max_gc,
        ) {
            Ok(n) => Ok(n),
            Err(n) => Err(n),
        }
    }
}

fn get_revcomp(s: &[u8]) -> Result<String, RuntimeError> {
    let seq = alphabets::dna::revcomp(s);
    match from_utf8(&seq) {
        Ok(n) => Ok(n.to_owned()),
        Err(n) => Err(RuntimeError::IncorrectDNASequence),
    }
}
