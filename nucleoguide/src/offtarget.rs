use crate::errors::RuntimeError;
use bio::alignment::pairwise;
use bio::alignment::Alignment;
use bio::alignment::AlignmentMode::*;
use bio::alignment::AlignmentOperation::*;
use std::collections::HashMap;

pub fn run(
    candidates: Vec<(&str, usize)>,
    reference: String,
    go: i32,
    ge: i32,
    minimum_mismatch: &usize,
) -> Result<(), RuntimeError> {
    let score = |a: u8, b: u8| if a == b { 1i32 } else { -1i32 };
    let aligner = pairwise::Aligner::new(go, ge, score);
    todo!()
}

// Run individual alignment against two sequences 
fn run_alignment<F: pairwise::MatchFunc>(
    s1: &str,
    s2: &str,
    mut aligner: pairwise::Aligner<F>,
    gf_size: &usize,
    minimum_mismatch: &usize,
) -> Option<Vec<Alignment>> {
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
            matches.push(i);
            reref = reref.split_at(end).1;
        }
    }
    if matches.is_empty() {
        None
    } else {
        Some(matches)
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
        assert_eq!(run_alignment("", "", aligner, &gf_size, &mm), None);
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
            run_alignment(s1, s2, aligner, &gf_size, &mm).unwrap().len(),
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
        assert_eq!(run, None)
    }
}
