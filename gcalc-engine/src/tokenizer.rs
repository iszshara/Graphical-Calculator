use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Plus,
    Minus,
    Multiply,
    Divide,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Lexer {
            chars: input.chars().peekable(),
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();
        while let Some(token) = self.next_token() {
            tokens.push(token);
        }
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let ch = self.chars.next()?;

        match ch {
            '+' => Some(Token::Plus),
            '-' => Some(Token::Minus),
            '*' => Some(Token::Multiply),
            '/' => Some(Token::Divide),

            '0'..='9' => {
                let mut number = ch.to_digit(10)? as i32;

                while let Some(next) = self.chars.peek() {
                    if let Some(digit) = next.to_digit(10) {
                        number = number * 10 + digit as i32;
                        self.chars.next(); // consume digit
                    } else {
                        break;
                    }
                }

                Some(Token::Number(number))
            }

            // Whitespace ignorieren
            c if c.is_whitespace() => self.next_token(),

            // Unbekanntes Zeichen
            _ => None,
        }
    }
}
