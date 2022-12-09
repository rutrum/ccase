use clap::{ArgAction, crate_version, Arg, Command, Error, error::ErrorKind, builder::StyledStr};
use crate::{CaseExtension};
use convert_case::{Case, Casing};

pub fn build() -> Command {
    Command::new("ccase")
        .version(crate_version!())
        .about("Convert between string cases.")
        .arg_required_else_help(true)
        .args(args::all())
        .override_usage(usage())
        .max_term_width(90)
        .after_help(after_help())
        .after_long_help(after_long_help())
}

fn usage() -> StyledStr {
    StyledStr::from(
        "\x1b[1mccase --to\x1b[0m <case> <input>...\n       \
         \x1b[1mccase --to\x1b[0m <case> \x1b[1m--from\x1b[0m <case> <input>...\n       \
         \x1b[1mccase --to\x1b[0m <case> \x1b[1m--boundaries\x1b[0m <string> <input>..."
    )
}

fn after_long_help() -> StyledStr {
    let s = format!(
        "\x1b[1;4mCases:\x1b[0m\n\
        {}\n\
        \x1b[1;4mConversion:\x1b[0m\n  \
        {}",
        list_cases(),
        conversion_description(),
    );
    
    StyledStr::from(s)
}

fn conversion_description() -> &'static str {
    "Case conversion is done in 3 steps.\n\
    \n  \
    \x1b[1mStep 1: Input is split into words by boundaries.\x1b[0m \n    \
    Boundaries identify how to split in the input string.  Boundaries can be the \n    \
    delimeters hyphen, underscore, and space.  Boundaries can identify splitting between\n    \
    characters based on character characteristics.  This includes a lowercase letter\n    \
    followed by an uppercase letter, a digit followed by an uppercase letter, etc.  After\n    \
    the input is split, we call the result words.\n\
    \n    \
    Boundaries are selected from those that join the `--from` case or any boundaries\n    \
    present in a string provided by `--boundaries`.
    \n  \
    \x1b[1mStep 2: Words are transformed to a certain pattern.\x1b[0m\n    \
    The list of words are transformed into lowercase, uppercase, or capitalized in a\n    \
    particular order.  The order of these transformations by word is called a pattern.\n\
    \n    \
    The pattern is select from that of the `--to` case.
    \n  \
    \x1b[1mStep 3: Transformed words are joined by a delimeter.\x1b[0m\n    \
    Finally the list of transformated words are joined by a delimeter, such as a hyphen\n    \
    underscore, or space.  There may also not be a delimeter at all, and words are\n    \
    concatenated directly together.\n\
    \n    \
    The delimiter is selected from the delimiter that joins the `--to` case.
    "
}

fn after_help() -> StyledStr {
    StyledStr::from(
        "\x1b[1;4mCases:\x1b[0m\n  See --help for list of cases
    ")
}

fn list_cases() -> String {
    let mut s = String::new();
    for case in Case::all_cases() {
        let case_str = format!("{:?}", case).to_case(Case::Flat);
        let underline_case = format!("\x1b[1m{}\x1b[0m", case_str);
        s = format!("{}{:>25}  {}\n", s, underline_case, case.name_in_case())
    }
    s
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
    //Err(Error::new(ErrorKind::DisplayHelp))
    Err(Error::raw(ErrorKind::ValueValidation, format!("'{}' is not a valid case.  See ccase --help for list of cases.", s))) 
}

mod args {
    use super::*;

    pub fn all() -> [Arg; 4] {
        [ to(), from(), input(), boundaries() ]
    }

    fn to() -> Arg {
        Arg::new("to")
            .short('t')
            .long("to")
            .value_name("case")
            .help("Case to convert to")
            .long_help("Convert the input into this case.  \
                The input is mutated and joined using the pattern and delimiter of the case.")
            .value_parser(case_value_parser)
            .required(true)
    }

    fn from() -> Arg {
        Arg::new("from")
            .short('f')
            .long("from")
            .value_name("case")
            .help("Case to parse input as")
            .long_help("Parse the input as if it were this case.  \
                This means splitting the input based on boundaries found in that case.")
            .value_parser(case_value_parser)
    }

    fn boundaries() -> Arg {
        Arg::new("boundaries")
            .short('b')
            .long("boundaries")
            .value_name("string")
            .help("String of boundaries to split input")
            .long_help("String that contains boundaries on how to split input.  \
                Any boundary contained in the string will be used as boundaries \
                for splitting input into words.")
            .conflicts_with("from")
    }

    fn input() -> Arg {
        Arg::new("input")
            .required(true)
            .help("The string(s) to convert")
            .long_help("The string(s) to convert into the --to case.")
            .action(ArgAction::Append)
    }
}
