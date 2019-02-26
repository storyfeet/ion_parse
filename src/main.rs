mod pesto;
use crate::pesto::Rule;
use clap::{clap_app, crate_version};

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

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let r = pesto::Command::parse(rule, &s).expect("Parse fail");
        println!("res = {:?}", r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_pass() {
        let v = vec![
            (Rule::Statement, "let x = 4"),
            (Rule::Statement, "for x in [0..4]; echo $x; end;"),
            (Rule::Statement, "for x in [0..4]\n echo $x\n end;"),
            (Rule::Range, "[0..4]"),
            (Rule::Range, "$(ls -l)"),
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
            (Rule::Statement, "let x & 4"),
            (Rule::Statement, "for x in ls -l; echo $x; end;"),
            (Rule::Statement, "for x in [0..4]\n echo $x\n end;"),
            (Rule::Range, "[0..Green]"),
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
