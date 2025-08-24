#[derive(Debug)]
pub enum Token {
    Word(Vec<u8>),
    Hex(Vec<u8>),
    Text(Vec<u8>),
    LeftParen,
    RightParen,
    Underscore,
    Indent,
    Dedent,
    Equal,
    Comma,
    Line,
}

pub enum State {
    None,
    Word,
    Line,
    Text,
    Hex,
}

pub fn lex(source: &[u8]) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut current: Vec<u8> = vec![];
    let mut state: State = State::None;

    let mut idents: Vec<usize> = vec![0];

    let mut index: usize = 0;

    while index <= source.len() {
        let byte = if index == source.len() { 0 } else { source[index] };

        match state {
            State::None => {
                current.clear();

                match byte {
                    b' ' | b'\t' => (),
                    b'\n' => state = State::Line,
                    b'(' => tokens.push(Token::LeftParen),
                    b')' => tokens.push(Token::RightParen),
                    b'=' => tokens.push(Token::Equal),
                    b',' => tokens.push(Token::Comma),
                    b'_' => tokens.push(Token::Underscore),
                    b'\"' => state = State::Text,
                    b'a'..=b'z' => {
                        current.push(byte);
                        state = State::Word;
                    }
                    b'0'..=b'9' | b'A'..=b'Z' => {
                        current.push(byte);
                        state = State::Hex;
                    }
                    0 => break,
                    _ => panic!("Unknown byte")
                }

                index += 1;
            }
            State::Text => {
                let next_byte = if index + 1 == source.len() { 0 } else { source[index + 1] };
                match (byte, next_byte) {
                    (b'\\', b'"') => {
                        current.push(b'\"');
                        index += 2;
                    }
                    (0, _) | (b'\"', _) => {
                        tokens.push(Token::Text(current.clone()));
                        index += 1;
                        state = State::None;
                    }
                    _ => {
                        index += 1;
                        current.push(byte);
                    }
                }
            },
            State::Line => match byte {
                b'\n' => {
                    current.clear();
                    index += 1;
                }
                b'\t' | b' ' => {
                    current.push(byte);
                    index += 1;
                }
                _ => {
                    if tokens.len() > 0 && byte != 0 {
                        let cur: usize = current.len();

                        if cur > *idents.last().unwrap() {
                            tokens.push(Token::Indent);
                            idents.push(cur);
                        } else if cur < *idents.last().unwrap() {
                            while cur < idents.pop().unwrap() {
                                tokens.push(Token::Dedent);
                            }
                        } else {
                            tokens.push(Token::Line);
                        }
                    }

                    state = State::None;
                    current.clear();
                },
            },
            State::Word => {
                match byte {
                    b'a'..=b'z' | b'0'..=b'9' => {
                        current.push(byte);
                        index += 1;
                    },
                    _ => {
                        tokens.push(Token::Word(current.clone()));
                        state = State::None;
                        current.clear();
                    },
                }
            }
            State::Hex => {
                match byte {
                    b'0'..=b'9' | b'A'..=b'Z' => {
                        current.push(byte);
                        index += 1;
                    },
                    _ => {
                        tokens.push(Token::Hex(current.clone()));
                        state = State::None;
                        current.clear();
                    },
                }
            }
        }
    }

    tokens
}

