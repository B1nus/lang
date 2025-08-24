mod lex;
mod parse;
use lex::*;
use parse::*;

fn main() {
    let tokens = lex(include_bytes!("txt"));
    let ast = parse(tokens);
}
