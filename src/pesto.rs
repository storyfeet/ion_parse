use pest_derive::*;

#[derive(Parser)]
#[grammar = "ion.pest"]
pub struct Command;
