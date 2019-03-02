mod pesto;
use crate::pesto::Rule;

use clap::{clap_app, crate_version};
use std::io::BufRead;

use pest::Parser;
fn main() {
    let cp = clap_app!(pegtest =>
    (version : crate_version!())
    (about : "Tests how pegs work")
    (author : "Matthew Stoodley,Zen3Ger")
    (@arg rule: -r +takes_value "The rule to look for -- default Statement")
    )
    .get_matches();

    let rule = match cp.value_of("rule").unwrap_or("Statement") {
        "Range" => Rule::Range,
        _ => Rule::Statement,
    };

    println!("Rule = {:?}", rule);

    let stdin = std::io::stdin();
    let mut stdin = stdin.lock();
    let mut buf = String::with_capacity(32);
    loop {
        stdin.read_line(&mut buf).ok();
        match pesto::Command::parse(rule, &buf) {
            Ok(res) => println!("res = {:?}", res),
            Err(e) => println!("{}", e),
        }
        buf.truncate(0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass() {
        let v = vec![
            (Rule::Main, "let x = 4"),
            (Rule::Statement, "echo $x"),
            (Rule::Main, "echo $x"),
            (Rule::Main, "for x in 0..4;echo $x; end;"),
            (Rule::Statement, "for x in 0..4\n echo $x\n end;"),
            (
                Rule::Statement,
                "for x y hotel in 0..100\n let b = \"$(x)oo\";echo b; end;",
            ),
            (Rule::Statement, "mayfail -p hello && isok"),
            (Rule::Statement, "echo $build(3 5 9)"),
            (Rule::Statement, "ls -l"),
            (Rule::Statement, "home/dir/"),
            (Rule::Statement, "./home/dir"),
            (Rule::Statement, "/dev/etc"),
            (Rule::Statement, "~/Documents/files"),
            (Rule::Statement, "cd ~/Documents/My\\ Pictures"),
            (Rule::Range, "0..4"),
            (Rule::Range, "0...4"),
            (Rule::Range, "0..3..9"),
            (Rule::Range, "10..-2..=0"),
            (Rule::Range, "$(ls -l)"),
            (Rule::Range, "0..$s"),
        ];

        let mut errs = Vec::new();
        for (n, (rl, st)) in v.iter().enumerate() {
            if let Err(e) = pesto::Command::parse(*rl, &st) {
                errs.push((n, rl, st, e));
            }
        }

        if errs.len() > 0 {
            for e in errs {
                println!("{:?}\n", e);
            }
            panic!();
        }
    }
    #[test]
    fn should_fail() {
        let v = vec![
            (Rule::Main, "let x & 4"),
            (Rule::Main, "for x in ls -l; echo $x; end;"),
            (Rule::Statement, "for x in [0..4]\n echo $x\n end;"),
            (Rule::Range, "[0..Green]"),
            (Rule::Range, "["),
            (Rule::Range, "$(ls -l)"),
        ];

        let mut errs = Vec::new();
        for (n, (rl, st)) in v.iter().enumerate() {
            if let Ok(v) = pesto::Command::parse(*rl, &st) {
                errs.push((n, rl, st, v));
            }
        }

        if errs.len() > 0 {
            for e in errs {
                println!("{:?}\n", e);
            }
            panic!();
        }
    }
}
