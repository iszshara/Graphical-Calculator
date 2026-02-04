use std::str::Chars;
use std::iter::Peekable;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i32),
    Float(f64),
    _Variable(char),
    Plus,
    Minus,
    Multiply,
    Divide,
    OpenParen,
    ClosedParen,
    Exponent,

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
            '(' => Some(Token::OpenParen),
            ')' => Some(Token::ClosedParen),
            '^' => Some(Token::Exponent),
            '0'..='9' => 
            {
                let mut int_value = ch.to_digit(10)? as i32;   // radix defines the number system

                // Loop for next digit
                while let Some(next_char) = self.chars.peek() 
                {
                    if next_char.is_ascii_digit()
                    {
                        int_value = int_value * 10 + next_char.to_digit(10)? as i32;
                        self.chars.next(); // Consume the digit
                    }
                    else
                    {
                        break;
                    }
                }

                if let Some('.') = self.chars.peek()
                {
                    // Connsumes the dot
                    self.chars.next();

                    let mut float_value = int_value as f64;
                    let mut decimal_place_factor = 0.1;
                    let mut has_fractional_digits = false;

                    while let Some(next_char) = self.chars.peek()
                    {
                        if next_char.is_ascii_digit()
                        {
                            has_fractional_digits = true;
                            float_value += next_char.to_digit(10)? as f64 * decimal_place_factor;
                            decimal_place_factor /= 10.0;
                            self.chars.next();
                        }
                        else 
                        {
                            break;    
                        }
                    }
                    Some(Token::Float(float_value))
                }
                else 
                {
                    Some(Token::Number(int_value))
                }

                
            },


            // Whitespace ignorieren
            c if c.is_whitespace() => self.next_token(),

            // Unbekanntes Zeichen
            _ => None,
        }
    }
}
