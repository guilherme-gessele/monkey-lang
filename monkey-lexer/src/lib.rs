use std::{iter::Peekable, str::Chars};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Illegal,
    EOF,

    // Identifiers
    Identifier(String),
    Integer(usize),

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    LeftParenthesis,
    RightParenthesis,
    LeftBrace,
    RightBrace,

    // Keywords
    Let,
    Function,
}

pub struct Lexer<'lexer> {
    input: Peekable<Chars<'lexer>>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(input: &'lexer str) -> Self {
        let input = input.chars().into_iter().peekable();
        Lexer { input }
    }

    fn read_until(&mut self, c: char, func: impl Fn(&char) -> bool) -> String {
        let mut output = vec![c];

        while let Some(c) = self.input.next_if(&func) {
            output.push(c);
        }

        String::from_iter(output)
    }
}

impl<'lexer> Iterator for Lexer<'lexer> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        let keyword = self.input.next();

        match keyword {
            Some('=') => Some(Token::Assign),
            Some('+') => Some(Token::Plus),
            Some('(') => Some(Token::LeftParenthesis),
            Some(')') => Some(Token::RightParenthesis),
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some(c) if c.is_digit(10) => {
                let str = self.read_until(c, |c| c.is_digit(10));

                Some(Token::Integer(str.parse::<usize>().unwrap()))
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let str = self.read_until(c, |c| c.is_ascii_alphabetic());

                match str.as_str() {
                    "let" => Some(Token::Let),
                    "fn" => Some(Token::Function),
                    _ => Some(Token::Identifier(str)),
                }
            }
            Some(c) if c.is_whitespace() => self.next(),
            Some(_) => Some(Token::Illegal),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use super::Token;

    #[test]
    fn delimiters() {
        let input = "=+(){},;";
        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::LeftParenthesis,
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::RightBrace,
            Token::Comma,
            Token::Semicolon,
        ];

        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(expected_tokens, tokens);
    }

    #[test]
    fn basic_code() {
        let input = "let five = 5;
        let ten = 10;
        let add = fn(x, y) {
        x + y;
        };
        let result = add(five, ten);
        ";

        let lexer = Lexer::new(input);

        let expected_tokens = vec![
            Token::Let,
            Token::Identifier(String::from("five")),
            Token::Assign,
            Token::Integer(5),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("ten")),
            Token::Assign,
            Token::Integer(10),
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("add")),
            Token::Assign,
            Token::Function,
            Token::LeftParenthesis,
            Token::Identifier(String::from("x")),
            Token::Comma,
            Token::Identifier(String::from("y")),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Identifier(String::from("x")),
            Token::Plus,
            Token::Identifier(String::from("y")),
            Token::Semicolon,
            Token::RightBrace,
            Token::Semicolon,
            Token::Let,
            Token::Identifier(String::from("result")),
            Token::Assign,
            Token::Identifier(String::from("add")),
            Token::LeftParenthesis,
            Token::Identifier(String::from("five")),
            Token::Comma,
            Token::Identifier(String::from("ten")),
            Token::RightParenthesis,
            Token::Semicolon,
        ];

        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(expected_tokens, tokens);
    }
}
