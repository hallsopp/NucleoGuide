use std::collections::HashSet;
use regex;
use crate::errors::errors::RuntimeError;

pub mod grnas {
    pub use super::run;
}

// Exposed funtion to search for guides.
// TODO: need this function to return a Vec<&str> of all 
// compatible guides.
pub fn run<'a>(s: &'a String, p: &String, gf_size: &usize, gf_xc: &bool, gf_xc_pattern: &String) -> Result<HashSet<&'a str>, RuntimeError> {
    let compiled_re = compile_re_pam(p)?;
    let temp_size = 20;
    let fw = re_pam_search(s, compiled_re)
        .and_then(|n| extract_grna_seq(s, n, &temp_size));
    if let Some(results) = fw {
        Ok(results)
    } else {
        Err(RuntimeError::NoGuidesFound)
    }
}

// Function to search using regex
fn re_pam_search(s: &String, re: regex::Regex) -> Option<Vec<usize>> {
    let mat: Vec<usize> = re.find_iter(s)
        .map(|n| n.start())
        .collect();
    if mat.len() < 1 {
        return None
    } else {
        return Some(mat)
    }
}

// Function to compile the PAM sequence into a regex expression 
fn compile_re_pam(p: &String) -> Result<regex::Regex, RuntimeError> {
    let n = regex::Regex::new("[nN]").unwrap();
    let modified_p = n.replace_all(p, "[agctAGCT]");
    let mut binding = regex::RegexBuilder::new(&modified_p);
    let init_p = binding.case_insensitive(true);
    match init_p.build() {
        Ok(n) => Ok(n),
        Err(_n) => Err(RuntimeError::InvalidPAM)
    }
}

fn compile_re_grna_exclusion(gf_xc: &String) -> Result<regex::Regex, RuntimeError> {
    let mut binding = regex::RegexBuilder::new(&gf_xc);
    let init_p = binding.case_insensitive(true);
    match init_p.build() {
        Ok(n) => Ok(n),
        Err(_n) => Err(RuntimeError::InvalidGRNAExclusionPattern)
    }
}

// fn to extract guide sequence from coordinates 
fn extract_grna_seq<'a>(s: &'a String, indexes: Vec<usize>, size: &usize) -> Option<HashSet<&'a str>> {
    let mut shortlist = HashSet::new();
    for pos in indexes.iter() {
        if pos < size {
            continue;
        } else {
            let start = pos - size;
            shortlist.insert(&s[start..*pos]);
        }
    };
    if shortlist.is_empty() {
        None
    } else {
        Some(shortlist)
    }
}

fn exclude_grna(mut candidate: HashSet<&str>, gf_xc: regex::Regex) -> HashSet<&str> {
    todo!()
}

#[cfg(test)]
mod tests {
    const CAS9_UPPER: &str = "NGG";
    const CAS9_LOWER: &str = "ngg";
    const GRNA_SIZE: usize = 20;
    use super::*;

    #[test]
    fn cas9_pam() {
        let result = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
    }
    #[test]
    fn cas9_lowercase_pam() {
        let result = compile_re_pam(&CAS9_LOWER.to_string()).unwrap();
    }
    #[test]
    fn basic_search() {
        let seq = String::from("AGCTTAGCTAGGA");
        let compiled_re = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
        let result = re_pam_search(&seq, compiled_re).unwrap();
        assert_eq!(result, [9])
    }
    #[test]
    fn basic_lowercase_search() {
        let seq = String::from("AGCTTAGCTAGGA");
        let compiled_re = compile_re_pam(&CAS9_LOWER.to_string()).unwrap();
        let result = re_pam_search(&seq, compiled_re).unwrap();
        assert_eq!(result, [9])
    }
    #[test]
    fn multiple_search() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGA");
        let compiled_re = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
        let result = re_pam_search(&seq, compiled_re).unwrap();
        assert_eq!(result, [9, 22, 35, 48])
    }
    #[test]
    fn single_str_select() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGA");
        let compiled_re = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 1)
    }
    #[test]
    fn multiple_str_select() {
        let seq = String::from("AGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAAGCTTAGCTAGGAACGCATGACTAGCATGCATGCATCGTACGTAGCTTTAAATCGATAGG");
        let compiled_re = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 2)
    }
    #[test]
    #[should_panic]
    fn fail_str_select() {
        let seq = String::from("AGCTTAGCTAGG");
        let compiled_re = compile_re_pam(&CAS9_UPPER.to_string()).unwrap();
        let index = re_pam_search(&seq, compiled_re).unwrap();
        let result = extract_grna_seq(&seq, index, &GRNA_SIZE).unwrap();
        assert_eq!(result.len(), 2)
    }
}