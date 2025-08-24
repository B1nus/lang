mod fun; // Functions
mod ocd; // Organizing
mod val; // Values
mod typ; // Types

use super::Token;

pub fn parse(tokens: Vec<Token>) -> () {
    let (main_tokens, funs_tokens) = fun::funs(tokens);
    let organized_main = ocd::organize(main_tokens);
    dbg!(organized_main);
}
