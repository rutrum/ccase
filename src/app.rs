use clap::{crate_version, Arg, Command, Error, error::ErrorKind, builder::StyledStr};
use crate::{CaseExtension};
use convert_case::{Case, Casing};

pub fn build() -> Command {
    Command::new("ccase")
        .version(crate_version!())
        .about("Converts between string cases.")
        .arg_required_else_help(true)
        .args(args::all())
        .override_usage(usage())
        .max_term_width(80)
        .after_help("See --help for a list of cases.")
        .after_long_help(list_cases())
}

fn usage() -> StyledStr {
    StyledStr::from(
        "\x1b[1mccase --to\x1b[0m <case> <input>\n       \
         \x1b[1mccase --to\x1b[0m <case> \x1b[1m--from\x1b[0m <case> <input>"
    )
}

fn list_cases() -> StyledStr {
    let mut s = String::from("\x1b[1;4mCases:\x1b[0m\n");
    for case in Case::all_cases() {
        let case_str = format!("{:?}", case).to_case(Case::Lower);
        let underline_case = format!("\x1b[1m{}\x1b[0m", case_str);
        s = format!("{}{:>25}  {}\n", s, underline_case, case.name_in_case())
    }
    StyledStr::from(s)
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
            .long_help("Convert the input into this case.  The input is mutated and joined using the pattern and delimiter of the case.")
            .value_parser(case_value_parser)
            .required(true)
    }

    fn from() -> Arg {
        Arg::new("from")
            .short('f')
            .long("from")
            .value_name("case")
            .help("Case to parse input as")
            .long_help("Parse the input as if it were this case.  This means splitting the input based on boundaries found in that case.")
            .value_parser(case_value_parser)
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
            .long_help("The string to convert into the --to case.")
    }
}
