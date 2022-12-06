use clap::{builder::TypedValueParser, crate_version, Arg, Command, Error, error::ErrorKind};
use crate::{CaseExtension};
use convert_case::{Case, Casing};
use std::ffi::OsStr;

pub fn build() -> Command {
    Command::new("ccase")
        .version(crate_version!())
        .about("Converts between string cases.")
        .arg_required_else_help(true)
        .args(args::all())
}

fn case_value_parser(s: &str) -> Result<Case, Error> {
    let case_str = s.to_case(Case::Flat);
    for case in Case::all_cases() {
        if case_str == format!("{:?}", case).to_case(Case::Flat) {
            return Ok(case);
        }
        if let Some(short) = case.short_name() {
            if case_str == short {
                return Ok(case);
            }
        }
    }
    Err(Error::new(ErrorKind::ValueValidation))
}

mod args {
    use super::*;

    pub fn all() -> [Arg; 3] {
        [ to(), from(), input() ]
    }

    fn to() -> Arg {
        Arg::new("to")
            .short('t')
            .long("to")
            .value_name("case")
            .help("Case to convert to")
            .value_parser(case_value_parser)
    }

    fn from() -> Arg {
        Arg::new("from")
            .short('f')
            .long("from")
            .value_name("case")
            .help("Case to parse input as")
            
    }

    /*
    fn boundaries() -> Arg {
        Arg::new("boundaries")
            .short('b')
            .long("boundaries")
            .help("String of boundaries to split input")
    }
    */

    fn input() -> Arg {
        Arg::new("input")
            .required(true)
            .help("The string to convert")
    }
}
