use ccase;
use convert_case::{Boundary, Case, Casing};
use clap::ArgMatches;
use std::env;
use std::io::{self, Read};

fn main() {
    let mut app = ccase::build_app();
            
    let missing_error = app.error(clap::error::ErrorKind::MissingRequiredArgument, 
        "The following required arguments were not provided:\n  \
            \x1b[32m<input>...\x1b[m");

    let args = get_args_with_stdin();

    let matches = app.clone().get_matches_from(args);

    let inputs = match matches.get_many::<String>("input") {
        None => if atty::isnt(atty::Stream::Stdin) {
            Default::default()
            //missing_error.exit();
        } else {
            missing_error.exit();
        }
        Some(inputs) => inputs,
    };
    
    inputs.for_each(|input| convert(&matches, input));
}

fn get_args_with_stdin() -> Vec<String> {
    let mut args: Vec<String> = env::args_os().map(|x| x.into_string().unwrap()).collect();

    if atty::isnt(atty::Stream::Stdin) {
        let stdin = io::stdin();
        let mut handle = stdin.lock();

        let mut v = Vec::new();
        handle.read_to_end(&mut v).unwrap();

        let s = String::from_utf8(v).unwrap();

        if !s.is_empty() {
            for word in s.lines() {
                args.push(word.trim_end().to_string());
            }
        }
    }

    args
}

fn convert(matches: &ArgMatches, input: &String) {
    let to = *matches.get_one::<Case>("to")
        .expect("--to is a required option");

    let result = if let Some(&from) = matches.get_one::<Case>("from") {
        input.from_case(from).to_case(to)
    } else if let Some(boundary_str) = matches.get_one::<String>("boundaries") {
        input.with_boundaries(&Boundary::list_from(boundary_str.as_str())).to_case(to)
    } else {
        input.to_case(to)
    };

    println!("{}", result);
}

#[cfg(test)]
mod test {
    use assert_cmd::{assert::Assert, Command};
    use predicates::str::contains;

    fn ccase(args: &[&str]) -> Assert {
        Command::cargo_bin("ccase")
            .unwrap()
            .args(args)
            .assert()
    }

    #[test]
    fn to_case() {
        ccase(&["-t", "snake", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["--to", "kebab", "myVarName"])
            .success()
            .stdout("my-var-name\n");
    }

    #[test]
    fn from_case() {
        ccase(&["-f", "snake", "-t", "pascal", "my_var-name"])
            .success()
            .stdout("MyVar-name\n");
        ccase(&["-t", "snake", "--from", "pascal", "myVar-name"])
            .success()
            .stdout("my_var-name\n");
    }

    #[test]
    fn to_required() {
        ccase(&["myvarname"])
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("--to"));
    }

    #[ignore] // atty is tricked in test, look at ccase -t snake manually
    #[test]
    fn input_required() {
        ccase(&["-t", "snake"])
            .failure()
            .stderr(contains("following required arguments"))
            .stderr(contains("input"));
    }

    #[test]
    fn help_default() {
        ccase(&[])
            .failure()
            .stderr(contains("Usage"));
    }

    #[test]
    fn case_inputs_not_lower() {
        ccase(&["-t", "SNAKE", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["-t", "SnAkE", "myVarName"])
            .success()
            .stdout("my_var_name\n");
        ccase(&["-t", "snake", "-f", "KEBab", "my-varName"])
            .success()
            .stdout("my_varname\n");
        ccase(&["-t", "snake", "-f", "KEBAB", "my-varName"])
            .success()
            .stdout("my_varname\n");
    }

    #[test]
    fn invalid_case() {
        ccase(&["-t", "SNEK", "myVarName"])
            .failure()
            .stderr(contains("Invalid value"))
            .stderr(contains("--to"));
        ccase(&["-t", "snake", "-f", "SNEK", "my-varName"])
            .failure()
            .stderr(contains("Invalid value"))
            .stderr(contains("--from"));
    }

    #[test]
    fn empty_string_input() {
        ccase(&["-t", "snake", r#""#])
            .success()
            .stdout("\n");
    }

    #[test]
    fn boundaries() {
        ccase(&["-t", "snake", "-b", "aA", "myVar-Name-Longer"])
            .success()
            .stdout("my_var-name-longer\n");
        ccase(&["-t", "snake", "-b", "-", "myVar-Name-Longer"])
            .success()
            .stdout("myvar_name_longer\n");
    }

    #[test]
    fn from_and_boundaries_exclusive() {
        ccase(&["-t", "snake", "-b", "_", "-f", "kebab", "myVar-Name-Longer"])
            .failure()
            .stderr(contains("--from"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--boundaries"));
    }

    #[test]
    fn multiple_inputs() {
        ccase(&["-t", "snake", "myVarName", "anotherMultiWordToken"])
            .success()
            .stdout("my_var_name\nanother_multi_word_token\n");
    }

    mod stdin {
        use super::*;

        fn pipe_ccase(stdin: &str, args: &[&str]) -> Assert {
            Command::cargo_bin("ccase")
                .unwrap()
                .args(args)
                .write_stdin(stdin)
                .assert()
        }
        
        #[test]
        fn stdin() {
            pipe_ccase("myVarName", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn newline_ending() {
            pipe_ccase("myVarName\n", &["-t", "snake"])
                .success()
                .stdout("my_var_name\n");
        }

        #[test]
        fn empty() {
            pipe_ccase(r#""#, &["-t", "snake"])
                .success()
                .stdout("");
        }

        #[test]
        fn multiple_inputs() {
            pipe_ccase("myVarName\nanotherMultiWordToken\n", &["-t", "Pascal"])
                .success()
                .stdout("MyVarName\nAnotherMultiWordToken\n");
        }
    }
}
