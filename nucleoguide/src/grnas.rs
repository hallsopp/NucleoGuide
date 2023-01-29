use crate::errors::RuntimeError;
use regex::{Regex, RegexBuilder};
use std::collections::HashSet;

// Exposed funtion to search for guides.
pub fn run<'a>(
    s: &'a str,
    p: &str,
    gf_size: &usize,
    gf_xc_pattern: &str,
    gf_ic_pattern: &str,
) -> Result<Vec<(&'a str, usize)>, RuntimeError> {
    let compiled_p = compile_re_pam_gfxc(p)?;
    let compiled_gf_xc = if !gf_xc_pattern.is_empty() {
        Some(compile_re_pam_gfxc(gf_xc_pattern)?)
    } else {
        None
    };
    let compiled_gf_ic = if !gf_ic_pattern.is_empty() {
        Some(compile_re_pam_gfxc(gf_ic_pattern)?)
    } else {
        None
    };
    let fw = run_thread(s, &compiled_p, gf_size, &compiled_gf_xc, &compiled_gf_ic);
    if let Some(results) = fw {
        Ok(results)
    } else {
        Err(RuntimeError::NoGuidesFound)
    }
}

fn run_thread<'a>(
    s: &'a str,
    comp_p: &Regex,
    gf_size: &usize,
    comp_xc: &Option<Regex>,
    comp_ic: &Option<Regex>,
) -> Option<Vec<(&'a str, usize)>> {
    let mut candidates = re_pam_search(s, comp_p).and_then(|x| extract_grna_seq(s, x, gf_size));
    if candidates.is_some() {
        if comp_xc.is_some() {
            candidates = exclude_grna(candidates.unwrap(), comp_xc.as_ref().unwrap());
        }
        if comp_ic.is_some() {
            candidates = include_grna(candidates.unwrap(), comp_ic.as_ref().unwrap());
        }
    } else {
        return None;
    }
    Some(candidates?)
}

// Function to search using regex
fn re_pam_search(s: &str, re: &Regex) -> Option<Vec<usize>> {
    let mat: Vec<usize> = re.find_iter(s).map(|n| n.start()).collect();
    if mat.is_empty() {
        None
    } else {
        Some(mat)
    }
}

// Function to compile the PAM sequence into a regex expression
fn compile_re_pam_gfxc(p: &str) -> Result<Regex, RuntimeError> {
    let n = Regex::new("[nN]").unwrap();
    let modified_p = n.replace_all(p, "[agctAGCT]");
    let mut binding = RegexBuilder::new(&modified_p);
    let init_p = binding.case_insensitive(true);
    match init_p.build() {
        Ok(n) => Ok(n),
        Err(_n) => Err(RuntimeError::InvalidPAM),
    }
}

// fn to extract guide sequence from coordinates
fn extract_grna_seq<'a>(
    s: &'a str,
    indexes: Vec<usize>,
    size: &usize,
) -> Option<Vec<(&'a str, usize)>> {
    let mut shortlist = Vec::new();
    let mut filter = HashSet::new();
    for pos in indexes.iter().enumerate() {
        if pos.1 < size {
            continue;
        } else {
            let start = *pos.1 - size;
            shortlist.push((&s[start..*pos.1], indexes[pos.0]));
        }
    }
    let fin: Vec<(&str, usize)> = shortlist
        .into_iter()
        .filter(|(s, _)| filter.insert(*s))
        .collect();
    if fin.is_empty() {
        None
    } else {
        Some(fin)
    }
}

fn exclude_grna<'a>(
    mut candidates: Vec<(&'a str, usize)>,
    gf_xc: &Regex,
) -> Option<Vec<(&'a str, usize)>> {
    candidates.retain(|x| !gf_xc.is_match(x.0));
    if candidates.is_empty() {
        None
    } else {
        Some(candidates)
    }
}

fn include_grna<'a>(
    mut candidates: Vec<(&'a str, usize)>,
    gf_ic: &Regex,
) -> Option<Vec<(&'a str, usize)>> {
    candidates.retain(|x| gf_ic.is_match(x.0));
    if candidates.is_empty() {
        None
    } else {
        Some(candidates)
    }
}

#[cfg(test)]
mod tests {
    const CAS9_UPPER: &str = "NGG";
    const CAS9_LOWER: &str = "ngg";
    const GRNA_SIZE: usize = 20;
    use super::*;

    #[test]
    fn cas9_pam() {
        let result = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
    }
    #[test]
    fn cas9_lowercase_pam() {
        let result = compile_re_pam_gfxc(&CAS9_LOWER.to_string()).unwrap();
    }
    #[test]
    fn basic_search() {
        let seq = String::from("AGCTTAGCTAGGA");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let result = re_pam_search(&seq, &compiled_re).unwrap();
        assert_eq!(result, [9])
    }
    #[test]
    fn basic_lowercase_search() {
        let seq = String::from("AGCTTAGCTAGGA");
        let compiled_re = compile_re_pam_gfxc(&CAS9_LOWER.to_string()).unwrap();
        let result = re_pam_search(&seq, &compiled_re).unwrap();
        assert_eq!(result, [9])
    }
    #[test]
    fn multiple_search() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGA");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let result = re_pam_search(&seq, &compiled_re).unwrap();
        assert_eq!(result, [9, 22, 35, 48])
    }
    #[test]
    fn single_str_select() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGA");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 1)
    }
    #[test]
    fn multiple_str_select() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 2)
    }
    #[test]
    #[should_panic]
    fn fail_str_select() {
        let seq = String::from("AGCTTAGCTAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 2)
    }
    #[test]
    fn grna_exclusion() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let compiled_gf_xc = compile_re_pam_gfxc(&"AGC".to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result_1 = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        let result = exclude_grna(result_1, &compiled_gf_xc);
        assert!(result.is_none())
    }
    #[test]
    fn grna_inclusion() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let compiled_gf_xc = compile_re_pam_gfxc(&"TTT".to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result_1 = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        let result = include_grna(result_1, &compiled_gf_xc);
        assert_eq!(result.unwrap().len(), 1)
    }
    #[test]
    fn grna_exclusion_with_n() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let compiled_gf_xc = compile_re_pam_gfxc(&"n".to_string()).unwrap();
        let index = re_pam_search(&seq, &compiled_re).unwrap();
        let result_1 = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        let result = exclude_grna(result_1, &compiled_gf_xc);
        assert!(result.is_none())
    }
    #[test]
    fn thread_test() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let compiled_gf_xc = Some(compile_re_pam_gfxc(&"TTT".to_string()).unwrap());
        let compiled_gf_ic = None;
        let result = run_thread(
            &seq,
            &compiled_re,
            &GRNA_SIZE,
            &compiled_gf_xc,
            &compiled_gf_ic,
        );
        assert_eq!(result.unwrap().len(), 1)
    }
    #[test]
    fn thread_test_2() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&CAS9_UPPER.to_string()).unwrap();
        let compiled_gf_xc = None;
        let compiled_gf_ic = Some(compile_re_pam_gfxc(&"TTTT".to_string()).unwrap());
        let result = run_thread(
            &seq,
            &compiled_re,
            &GRNA_SIZE,
            &compiled_gf_xc,
            &compiled_gf_ic,
        );
        assert!(result.is_none())
    }
    #[test]
    fn thread_test_3() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam_gfxc(&"TTTT".to_string()).unwrap();
        let compiled_gf_xc = None;
        let compiled_gf_ic = Some(compile_re_pam_gfxc(&"TTTT".to_string()).unwrap());
        let result = run_thread(
            &seq,
            &compiled_re,
            &GRNA_SIZE,
            &compiled_gf_xc,
            &compiled_gf_ic,
        );
        assert!(result.is_none())
    }
}
