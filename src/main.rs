use ccase::{self, CaseExtension};
use convert_case::{Boundary, Case, Casing};

fn main() {
    let app = ccase::build_app();
    let matches = app.get_matches();

    let input = matches.get_one::<String>("input")
        .expect("input is a required argument");
    
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
    fn not_from_and_boundaries() {
        ccase(&["-t", "snake", "-b", "_", "-f", "kebab", "myVar-Name-Longer"])
            .failure()
            .stderr(contains("--from"))
            .stderr(contains("cannot be used with"))
            .stderr(contains("--boundaries"));
    }
}
