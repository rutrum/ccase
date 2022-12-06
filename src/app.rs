use clap::{crate_version, Arg, Command};

pub fn build() -> Command {
    Command::new("ccase")
        .version(crate_version!())
        .about("Converts between string cases.")
        .arg_required_else_help(true)
        .args(args::all())
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
            .help("The string to convert")
    }
}
