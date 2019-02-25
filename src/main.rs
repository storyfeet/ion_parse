mod pesto;
use crate::pesto::Rule;
use pest::Parser;
fn main() {
    println!("Hello, world!");

    loop {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).ok();
        let r = pesto::Command::parse(Rule::Statement, &s).expect("Parse fail");
        println!("res = {:?}", r);
    }
}
