use crate::errors::RuntimeError;
use bio::alphabets;
use grnas::Grna;
use offtarget::OffTargetList;
use std::str::from_utf8;
mod errors;
mod grnas;
mod offtarget;

#[derive(Debug)]
pub struct GuideDesign {
    seq: String,
    ot_search: String, // this option will be depreciated when whole genome searching is implemented
    revcomp: String,
    pam: String,
    gf_size: usize,
    gf_xc_pattern: String,
    gf_ic_pattern: String,
    gf_min_gc: f32,
    gf_max_gc: f32,
    ot_go: i32, // potentially split up the guide and off-target features 
    ot_ge: i32,
    ot_mm: usize,
}

impl GuideDesign {
    // instantise a new guide design object
    pub fn new(
        s: String,
        ots: String,
        p: String,
        gf_size: usize,
        gf_xc_pattern: String,
        gf_ic_pattern: String,
        gf_min_gc: f32,
        gf_max_gc: f32,
        ot_go: i32,
        ot_ge: i32,
        ot_mm: usize
    ) -> Result<Self, RuntimeError> {
        let check_s = s.as_bytes();
        let check_ots = ots.as_bytes();
        let dna_ab = alphabets::dna::alphabet();
        // check that input seqeunce is valid DNA
        if !dna_ab.is_word(check_s) || !dna_ab.is_word(check_ots){
            Err(RuntimeError::IncorrectDNASequence)
        } else {
            let rc = get_revcomp(check_s)?;
            Ok(GuideDesign {
                seq: s,
                ot_search: ots,
                revcomp: rc,
                pam: p,
                gf_size,
                gf_xc_pattern,
                gf_ic_pattern, // need to tidy these up, can use shorthand
                gf_min_gc,
                gf_max_gc,
                ot_go,
                ot_ge,
                ot_mm,
            })
        }
    }
    // Funtion to search for grnas
    pub fn idgrnas(&self) -> Result<Vec<Grna>, RuntimeError> {
        grnas::run(
            &self.seq,
            &self.revcomp,
            &self.pam,
            &self.gf_size,
            &self.gf_xc_pattern,
            &self.gf_ic_pattern,
            &self.gf_min_gc,
            &self.gf_max_gc,
        ) 
    }
    // Function to search for off-targets 
    pub fn idofftargets<'a>(&self, grnas: Vec<Grna<'a>>) -> Result<Option<Vec<OffTargetList<'a>>>, RuntimeError> {
        offtarget::run(grnas, &self.ot_search, &self.gf_size, &self.ot_go, &self.ot_ge, &self.ot_mm)
    }
}

fn get_revcomp(s: &[u8]) -> Result<String, RuntimeError> {
    let seq = alphabets::dna::revcomp(s);
    match from_utf8(&seq) {
        Ok(n) => Ok(n.to_owned()),
        Err(_n) => Err(RuntimeError::IncorrectDNASequence),
    }
}
