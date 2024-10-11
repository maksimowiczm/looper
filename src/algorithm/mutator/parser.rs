use crate::algorithm::mutator::best_mutator::BestMutator;
use crate::algorithm::mutator::current_mutator::CurrentMutator;
use crate::algorithm::mutator::random_mutator::RandomMutator;
use crate::algorithm::mutator::Mutate;
use std::fmt::{Display, Formatter};
use std::io::Read;
use std::iter::Peekable;

// region -- Parser --

#[derive(Debug)]
pub enum ParserError {
    UnexpectedEndOfInput,
    UnexpectedToken {
        unexpected: Token,
        expected: Token,
        near_tokens: Vec<Token>,
    },
    UnknownMutator,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ParserError::UnexpectedEndOfInput => write!(f, "Unexpected end of input"),
            ParserError::UnexpectedToken {
                unexpected,
                expected,
                near_tokens,
            } => {
                write!(
                    f,
                    "Unexpected token: {:?}, expected: {:?}, near tokens: {:?}",
                    unexpected, expected, near_tokens
                )
            }
            ParserError::UnknownMutator => write!(f, "Unknown mutator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Slash,
    DE,
    Word(String),
    Number(u32),
}

pub fn parse_mutator(mutator: &str) -> Result<Box<dyn Mutate>, ParserError> {
    let mut tokens = tokenize(mutator).into_iter().peekable();
    parse(&mut tokens)
}

fn parse(
    tokens: &mut Peekable<impl Iterator<Item = Token>>,
) -> Result<Box<dyn Mutate>, ParserError> {
    let x: String; // special thanks to our professor for naming this variable (it a satire please don't fail us 😔)
    let how_many: usize;
    let scheme: String;

    // skip DE token
    if skip_token(tokens, Token::DE) {
        skip_token(tokens, Token::Slash);
    }

    // parse x
    match tokens.next() {
        Some(Token::Word(word)) => x = word,
        Some(token) => {
            return Err(ParserError::UnexpectedToken {
                unexpected: token.clone(),
                expected: Token::Word("<x>".to_string()),
                near_tokens: [token].into_iter().chain(tokens.take(3)).collect(),
            })
        }
        None => return Err(ParserError::UnexpectedEndOfInput),
    }
    expect_token(tokens, Token::Slash)?;

    // parse how_many
    match tokens.next() {
        Some(Token::Number(value)) => how_many = value as usize,
        Some(token) => {
            return Err(ParserError::UnexpectedToken {
                unexpected: token.clone(),
                expected: Token::Number(0),
                near_tokens: [token].into_iter().chain(tokens.take(3)).collect(),
            })
        }
        None => return Err(ParserError::UnexpectedEndOfInput),
    }
    expect_token(tokens, Token::Slash)?;

    // parse scheme
    match tokens.next() {
        Some(Token::Word(word)) => scheme = word,
        Some(token) => {
            return Err(ParserError::UnexpectedToken {
                unexpected: token.clone(),
                expected: Token::Word("<scheme>".to_string()),
                near_tokens: [token].into_iter().chain(tokens.take(3)).collect(),
            })
        }
        None => return Err(ParserError::UnexpectedEndOfInput),
    }

    let mutator: Box<dyn Mutate> = match (x.as_str(), how_many, scheme.as_str()) {
        ("current", how_many, "bin") => Box::new(CurrentMutator { how_many }),
        ("best", how_many, "bin") => Box::new(BestMutator { how_many }),
        ("random", how_many, "bin") => Box::new(RandomMutator { how_many }),
        _ => return Err(ParserError::UnknownMutator),
    };

    Ok(mutator)
}

// endregion

// region -- Lexer --

/// Skip allowed tokens
/// Returns true if the token was skipped
fn skip_token(tokens: &mut Peekable<impl Iterator<Item = Token>>, allow: Token) -> bool {
    if let Some(next) = tokens.peek() {
        if *next == allow {
            tokens.next();
            return true;
        }
    }

    false
}

/// Expect a token and consume it
fn expect_token(
    tokens: &mut impl Iterator<Item = Token>,
    expected: Token,
) -> Result<(), ParserError> {
    let next = tokens.next().ok_or(ParserError::UnexpectedEndOfInput)?;

    if next == expected {
        Ok(())
    } else {
        Err(ParserError::UnexpectedToken {
            unexpected: next,
            expected,
            near_tokens: tokens.take(3).collect(),
        })
    }
}

fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        match ch {
            '/' => tokens.push(Token::Slash),
            '0'..='9' => {
                let mut number = ch.to_string();
                while let Some(&ch) = chars.peek() {
                    if ch.is_ascii_digit() {
                        number.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()));
            }
            _ => {
                let mut word = ch.to_string();
                while let Some(&ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        word.push(ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                if word == "DE" {
                    tokens.push(Token::DE);
                } else {
                    tokens.push(Token::Word(word));
                }
            }
        }
    }

    tokens
}

// endregion

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "DE/best/1/bin",
        &[
            Token::DE,
            Token::Slash,
            Token::Word("best".to_string()),
            Token::Slash,
            Token::Number(1),
            Token::Slash,
            Token::Word("bin".to_string())
        ]
    )]
    fn test_tokenize(#[case] input: &str, #[case] expected: &[Token]) {
        assert_eq!(tokenize(input), expected);
    }
}
