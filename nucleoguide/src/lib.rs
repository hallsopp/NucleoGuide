use crate::errors::RuntimeError;
use crate::grnas::run;
use bio::alphabets;
use grnas::Grna;
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
    gf_size: usize,
    gf_xc_pattern: String,
    gf_ic_pattern: String,
    gf_min_gc: f32,
    gf_max_gc: f32,
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
                gf_size: gf_size,
                gf_xc_pattern: gf_xc_pattern,
                gf_ic_pattern: gf_ic_pattern,
                gf_min_gc: gf_min_gc,
                gf_max_gc: gf_max_gc,
            })
        }
    }
    // Funtion to search for grnas
    pub fn idgrnas(&self) -> Result<Vec<Grna>, RuntimeError> {
        match run(
            &self.seq,
            &self.revcomp,
            &self.pam,
            &self.gf_size,
            &self.gf_xc_pattern,
            &self.gf_ic_pattern,
            &self.gf_min_gc,
            &self.gf_max_gc,
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
