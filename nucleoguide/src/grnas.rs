use regex;
use crate::errors::errors::RuntimeError;

pub mod grnas {
    use crate::errors::errors::RuntimeError;
    use super::re_pam_search;

    // Exposed funtion to search for guides.
    // TODO: need this function to return a Vec<&str> of all 
    // compatible guides.
    pub fn find(s: &String, p: &String, _po: &usize) -> Result<usize, RuntimeError> {
        match re_pam_search(s, p) {
            Ok(n) => Ok(n),
            Err(n) => Err(n)
        }
    }
}

// Function to search using regex
// TODO: need to return a Vec<usize> of starts of all matches 
// TODO: need to apply the PAM offset here, for example operate 
// -1 on these indexes.
fn re_pam_search(s: &String, p: &String) -> Result<usize, RuntimeError> {
    let re = compile_re(p)?;
    match re.find(s) {
        Some(n) => Ok(n.start()),
        None => Err(RuntimeError::NoGuidesFound)
    }
}

// Function to compile the PAM sequence into a regex expression 
// TODO: need to pattern match U -> [atcgATCG]
fn compile_re(p: &String) -> Result<regex::Regex, RuntimeError> {
    match regex::Regex::new(p) {
        Ok(n) => Ok(n),
        Err(_n) => Err(RuntimeError::InvalidPAM)
    }
}

// TODO: need to write function to search through sequence 
// and extract 20 bases etc behind
fn extract_grna_seq(s: &String, indexes: Vec<usize>) -> Result<Vec<&str>, RuntimeError> {
    todo!()
}