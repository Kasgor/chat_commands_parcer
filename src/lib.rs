use pest_derive::Parser;
//use pest::{iterators::Pair, Parser};
// Не зрозумів як використати тут thiserror
#[derive(Parser)]
#[grammar = "grammar.pest"]
pub struct Grammar;
