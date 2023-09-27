mod parser;

const SOURCE: &'static str = include_str!("../test.txt");

fn main() {
    let mut lexer = parser::Lexer::new("<stdin>", SOURCE);
    let res = lexer.next_token();
    println!("{:?}", res);
}
