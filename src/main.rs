use ccase::{self, CaseExtension};
use convert_case::{Case, Casing};

fn main() {
    let app = ccase::build_app();
    let matches = app.get_matches();

    let input = matches.get_one::<String>("input")
        .expect("input is a required argument");
    
    let to = *matches.get_one::<Case>("to")
        .expect("--to is a required option");

    let result = if let Some(&from) = matches.get_one::<Case>("from") {
        input.from_case(from).to_case(to)
    } else {
        input.to_case(to)
    };

    println!("{}", result);

}

#[cfg(test)]
mod test {
    use assert_cmd::{assert::Assert, Command};
    use predicates::prelude::*;

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
    }

    #[test]
    fn from_case() {
        ccase(&["-f", "snake", "-t", "pascal", "my_var-name"])
            .success()
            .stdout("MyVar-name\n");
    }
}
