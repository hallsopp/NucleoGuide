// custom module for errors
use std::fmt;

#[derive(Debug)]
pub enum RuntimeError {
    IncorrectDNASequence,
    NoGuidesFound,
    InvalidPAM,
    InvalidGRNAExclusionPattern,
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::IncorrectDNASequence => write!(f, "Incorrect DNA sequence provided."),
            RuntimeError::NoGuidesFound => write!(f, "No compatible gRNAs found."),
            RuntimeError::InvalidPAM => write!(f, "PAM Sequence is not valid."),
            RuntimeError::InvalidGRNAExclusionPattern => {
                write!(f, "gRNA exclusion pattern is not valid.")
            }
        }
    }
}
