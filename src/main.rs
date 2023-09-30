#![allow(dead_code)]

mod parser;

const SOURCE: &'static str = include_str!("../test.txt");

use crate::parser::Lexer;

fn main() {
    let mut lexer = Lexer::new("<stdin>", SOURCE);
    let res = lexer.try_parse_tokens();
    println!("{:#?}", res);
}
