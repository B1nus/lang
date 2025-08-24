use super::Token;

// Grouping tokens

#[derive(Debug)]
pub enum Code {
    Loop {
        name: Vec<u8>,
        cond: Vec<Token>,
        body: Box<Vec<Code>>,
    },
    Set {
        names: Vec<Option<Vec<u8>>>,
        val: Vec<Token>,
    },
    Call(Vec<Token>),
    Use(Vec<Token>),
    Case {
        arg: Vec<Token>,
        cases: Vec<(Vec<Token>, Vec<Code>)>,
    },
    Give(Vec<Token>),
    Stop(Vec<Token>),
    Next(Vec<Token>),
    Enum(Vec<Token>),
    Data(Vec<Token>),
}

pub fn collect_until_line(tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut out = vec![];

    while tokens.len()
}

pub fn collect_until_right_paren(tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut out = vec![];

    let mut c: usize = 1;

    while c > 0 {
        match tokens.
    }
}

pub fn collect_until_dedent(tokens: &mut Vec<Token>) -> Vec<Token> {
    let mut out = Vec![];

    let mut c: usize = 1;
}

pub fn organize(mut tokens: Vec<Token>) -> Vec<Code> {
    let mut code = vec![];

    tokens.reverse();

    let mut dent: isize = 1;

    while tokens.len() > 0 {
        match tokens.pop() {
            Some(Token::Word(word)) => match std::str::from_utf8(&word).unwrap() {
                "loop" => _ = dbg!("hello"),
                "use" => match tokens.pop() {
                    Some(Token::LeftParen) => {
                        let mut call = vec![Token::Word(word), Token::LeftParen];
                        call.extend(collect_until_right_paren(&mut tokens));
                    }
                "case" => _ = dbg!("hello"),
                "give" => _ = dbg!("hello"),
                "stop" => _ = dbg!("hello"),
                "next" => _ = dbg!("hello"),
                "data" => _ = dbg!("hello"),
                "enum" => _ = dbg!("hello"),
                _ => _ = dbg!(word),
            }
            Some(Token::Dedent) | None => break,
            _ => panic!(),
        }
    }

    code
}

