use super::Token;

// Grouping tokens into functions

#[derive(Debug)]
pub struct Fun {
    pub name: Vec<u8>,
    pub pars: Vec<Token>,
    pub rets: Vec<Token>,
    pub body: Vec<Token>,
}

pub fn funs(mut tokens: Vec<Token>) -> (Vec<Token>, Vec<Fun>) {
    let mut main = vec![];
    let mut funs = vec![];

    tokens.reverse();

    loop {
        match (tokens.pop(), tokens.pop()) {
            (Some(Token::Word(name)), Some(Token::LeftParen)) => {
                let mut inside = vec![];

                let mut p: isize = 1;
                while p > 0 {
                    let token = tokens.pop();
                    match &token {
                        Some(Token::LeftParen) => p += 1,
                        Some(Token::RightParen) => p -= 1,
                        None => todo!(),
                        _ => (),
                    }
                    inside.push(token.unwrap());
                }
                _ = inside.pop();

                let mut outside = vec![];

                loop {
                    match tokens.pop() {
                        Some(Token::Dedent) => todo!(),
                        Some(Token::Line) | None => {
                            main.push(Token::Word(name));
                            main.push(Token::LeftParen);
                            main.extend(inside);
                            main.push(Token::RightParen);
                            main.extend(outside);
                            break;
                        }
                        Some(Token::Indent) => {
                            let mut body = vec![];
                            let mut i_count: usize = 1;
                            while i_count > 0 {
                                let token = tokens.pop();
                                match &token {
                                    Some(Token::Indent) => i_count += 1,
                                    Some(Token::Dedent) => i_count -= 1,
                                    None => break,
                                    _ => (),
                                }
                                body.push(token.unwrap());
                            }
                            funs.push(Fun {
                                name: name,
                                pars: inside,
                                rets: outside,
                                body: body,
                            });
                            break;
                        },
                        Some(token) => outside.push(token),
                    }
                }
            }
            (None, _) => break,
            (Some(token), None) => main.push(token),
            (Some(token), Some(token2)) => {
                main.push(token);
                main.push(token2);
            },
        }
    }

    return (main, funs);
}
