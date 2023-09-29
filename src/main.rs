#![allow(dead_code)]

mod parser;

const SOURCE: &'static str = include_str!("../test.txt");

use crate::parser::*;

fn main() {
    let mut lexer = parser::Lexer::new("<stdin>", SOURCE);
    let res = lexer.try_parse_tokens();
    println!("{:#?}", res);
}
