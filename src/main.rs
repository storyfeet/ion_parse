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
