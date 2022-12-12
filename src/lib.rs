mod app;
mod case_extension;

pub use app::build as build_app;
pub use case_extension::*;

use clap::error::{Error, ErrorKind};
use convert_case::{Case, Casing, Pattern};

fn pattern_value_parser(s: &str) -> Result<Pattern, Error> {
    let pattern_str = s.to_case(Case::Flat);
    for pattern in all_patterns() {
        let pattern_in_flat = format!("{:?}", pattern).to_case(Case::Flat);
        if pattern_str == pattern_in_flat {
            return Ok(pattern);
        }
    }
    Err(Error::raw(ErrorKind::ValueValidation, format!("'{}' is not a valid pattern.  See ccase --help for list of patterns.", s))) 
}

fn all_patterns() -> Vec<Pattern> {
    use Pattern::*;
    vec![
        Lowercase,
        Uppercase,
        Capital,
        Sentence,
        Camel,
        Alternating,
        Toggle,
        PseudoRandom,
        Random,
    ]
}

fn case_value_parser(s: &str) -> Result<Case, Error> {
    let case_str = s.to_case(Case::Flat);
    for case in Case::all_cases() {
        if case_str == case.name_to_flat_case() {
            return Ok(case);
        }
        if let Some(short) = case.short_name() {
            if case_str == short {
                return Ok(case);
            }
        }
    }
    Err(Error::raw(ErrorKind::ValueValidation, format!("'{}' is not a valid case.  See ccase --help for list of cases.", s))) 
}
