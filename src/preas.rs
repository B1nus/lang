use super::Token;

#[derive(Debug)]
pub enum Code {
    Set(Set),
    Val(Val),
}

#[derive(Debug)]
pub struct Set {
    name: Vec<u8>,
    val: Val,
}

#[derive(Debug)]
pub enum Val {
    Hex(Vec<u8>),
    Name(Vec<u8>),
}

pub fn val(tokens: &mut &[Token]) -> Val {
    let val = match next(tokens) {
        Some(Token::Hex(hex)) => Val::Hex(hex.clone()),
        Some(Token::Word(word)) => Val::Name(word.clone()),
        _ => panic!(),
    };

    return val;
}

pub fn next(tokens: &mut &[Token]) -> Option<Token> {
    if tokens.len() > 0 {
        *tokens = &tokens[1..];
    }
    return peek(tokens);
}

pub fn peek(tokens: &[Token]) -> Option<Token> {
    match tokens.len() {
        0.. => Some(tokens[0].clone()),
        _ => None,
    }
}

pub fn parse(tokens: &[Token]) -> Vec<Code> {
    let mut code: Vec<Code> = vec![];

    let mut tokens: &[Token] = tokens;

    while tokens.len() > 0 {
        match tokens[0] {
            Token::Word(word) => set(&tokens[1..]),
        }
        dbg!(next(&mut tokens));
    }

    code
}
