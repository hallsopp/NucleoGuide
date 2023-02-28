use crate::errors::RuntimeError;
use crate::grnas::Grna;
use bio::alignment::pairwise;
use bio::alignment::Alignment;
use bio::alignment::AlignmentMode::*;
use bio::alignment::AlignmentOperation::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct OffTargetList<'a> {
    guide: Grna<'a>,
    off_targets: Vec<OffTarget>
}

impl<'a> OffTargetList<'a> {
    pub fn new(guide: Grna<'a>, off_targets: Vec<OffTarget>) -> Self {
        OffTargetList { guide, off_targets }
    }
}

#[derive(Debug)]
pub struct OffTarget {
    score: i32,
    ystart: usize,
    yend: usize,
}

impl OffTarget {
    pub fn new(score: i32, ystart: usize, yend: usize) -> Self {
        OffTarget { score, ystart, yend }
    }
}

pub fn run<'a>(
    candidates: Vec<Grna<'a>>,
    reference: &str,
    gf_size: &usize,
    go: &i32,
    ge: &i32,
    minimum_mismatch: &usize,
) -> Result<Option<Vec<OffTargetList<'a>>>, RuntimeError> {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let mut aligner = pairwise::Aligner::new(*go, *ge, score);
    let mut result = Vec::new();
    for cand in candidates {
        let (new_aligner, ota) = run_alignment(cand.get_seq(), reference, aligner, gf_size, minimum_mismatch);
        aligner = new_aligner;
        if let Some(ota_list) = ota {
            let ota_list = OffTargetList::new(cand, ota_list);
            result.push(ota_list);
        }
    }
    if result.is_empty() {
        Ok(None)
    } else {
        Ok(Some(result))
    }
}

// Run individual alignment against two sequences
fn run_alignment<'a, F: pairwise::MatchFunc>(
    s1: &'a str,
    s2: &'a str,
    mut aligner: pairwise::Aligner<F>,
    gf_size: &'a usize,
    minimum_mismatch: &'a usize,
) -> (pairwise::Aligner<F>, Option<Vec<OffTarget>>) {
    let mut matches = Vec::new();
    let mut reref = s2;
    let desired = *gf_size - minimum_mismatch;
    loop {
        if reref.len() < desired {
            break;
        }
        let i = aligner.semiglobal(s1.as_bytes(), reref.as_bytes());
        let end = i.yend;
        if i.score == 0 {
            break;
        } else if (i.score as usize) < desired {
            reref = reref.split_at(end).1;
        } else {
            let mat = OffTarget::new(i.score, i.ystart, end);
            matches.push(mat);
            reref = reref.split_at(end).1;
        }
    }
    if matches.is_empty() {
        (aligner, None)
    } else {
        (aligner, Some(matches))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_strings() {
        let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
        let aligner = pairwise::Aligner::new(-1, -1, score);
        let gf_size = 5;
        let mm = 1;
        assert!(run_alignment("", "", aligner, &gf_size, &mm).1.is_none());
    }
    #[test]
    fn identical_dna_strings() {
        let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
        let aligner = pairwise::Aligner::new(-1, -1, score);
        let gf_size = 5;
        let mm = 1;
        let s1 = "ATCG";
        let s2 = "ATCG";
        assert_eq!(
            run_alignment(s1, s2, aligner, &gf_size, &mm).1.unwrap().len(),
            1
        );
    }
    #[test]
    fn different_dna_strings_with_matching_substring() {
        let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
        let aligner = pairwise::Aligner::new(-1, -1, score);
        let gf_size = 5;
        let mm = 1;
        let s1 = "ATCGAATT";
        let s2 = "CAATTGAG";
        let run = run_alignment(s1, s2, aligner, &gf_size, &mm);
        assert!(run.1.is_none())
    }
}
