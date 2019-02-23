mod pesto;
use pest::Parser;
use crate::pesto::Rule;
fn main() {
    println!("Hello, world!");


    loop{
        let mut s = String::new(); 
        std::io::stdin().read_line(&mut s).ok();
        let r = pesto::Command::parse(Rule::Statement,&s).expect("Parse fail");
        println!("res = {:?}",r);
    }

}
