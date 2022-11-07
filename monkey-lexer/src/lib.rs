use std::{iter::Peekable, str::Chars};

const ASSIGN: char = '=';
const PLUS: char = '+';
const MINUS: char = '-';
const BANG: char = '!';
const SLASH: char = '/';
const ASTERISK: char = '*';
const LESS_THAN: char = '<';
const GREATER_THAN: char = '>';

const LEFT_PARENTHESIS: char = '(';
const RIGHT_PARENTHESIS: char = ')';
const LEFT_BRACE: char = '{';
const RIGHT_BRACE: char = '}';
const COMMA: char = ',';
const SEMICOLON: char = ';';

const FUNCTION: &'static str = "fn";
const LET: &'static str = "let";
const TRUE: &'static str = "true";
const FALSE: &'static str = "false";
const IF: &'static str = "if";
const ELSE: &'static str = "else";
const RETURN: &'static str = "return";

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
    Minus,
    Bang,
    Slash,
    Asterisk,
    LessThan,
    GreatherThan,
    Equals,
    NotEquals,

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
    If,
    Else,
    Return,
    True,
    False,
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
            Some(ASSIGN) => {

                if let Some(p) = self.input.peek() {
                    
                    if p == &ASSIGN {
                        self.input.next();
                        return Some(Token::Equals);
                    }
                    
                    return Some(Token::Assign);
                }

                Some(Token::Illegal)
            },
            Some(PLUS) => Some(Token::Plus),
            Some(MINUS) => Some(Token::Minus),
            Some(BANG) => {

                if let Some(p) = self.input.peek() {
                    
                    if p == &ASSIGN {
                        self.input.next();
                        return Some(Token::NotEquals);
                    }
                    
                    return Some(Token::Bang);
                }

                Some(Token::Illegal)
            } ,
            Some(SLASH) => Some(Token::Slash),
            Some(ASTERISK) => Some(Token::Asterisk),
            Some(LESS_THAN) => Some(Token::LessThan),
            Some(GREATER_THAN) => Some(Token::GreatherThan),

            Some(LEFT_PARENTHESIS) => Some(Token::LeftParenthesis),
            Some(RIGHT_PARENTHESIS) => Some(Token::RightParenthesis),
            Some(LEFT_BRACE) => Some(Token::LeftBrace),
            Some(RIGHT_BRACE) => Some(Token::RightBrace),
            Some(COMMA) => Some(Token::Comma),
            Some(SEMICOLON) => Some(Token::Semicolon),
            Some(c) if c.is_digit(10) => {
                let str = self.read_until(c, |c| c.is_digit(10));

                Some(Token::Integer(str.parse::<usize>().unwrap()))
            }
            Some(c) if c.is_ascii_alphabetic() => {
                let str = self.read_until(c, |c| c.is_ascii_alphabetic());

                match str.as_str() {
                    LET => Some(Token::Let),
                    FUNCTION => Some(Token::Function),
                    TRUE => Some(Token::True),
                    FALSE => Some(Token::False),
                    IF => Some(Token::If),
                    ELSE => Some(Token::Else),
                    RETURN => Some(Token::Return),
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

    #[test]
    fn extended_keywords_and_operators() {
        let input = "let five = 5;
        
        let ten = 10;
        
        let add = fn(x, y) {
            x + y;
        };

        let result = add(five, ten);
        
        !-/*5;
        5 < 10 > 5;

        if (5 < 10) {
            return true;
        } else {
            return false;
        }

        10 == 10;

        10 != 9;
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

            Token::Bang,
            Token::Minus,
            Token::Slash,
            Token::Asterisk,
            Token::Integer(5),
            Token::Semicolon,

            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::GreatherThan,
            Token::Integer(5),
            Token::Semicolon,

            Token::If,
            Token::LeftParenthesis,
            Token::Integer(5),
            Token::LessThan,
            Token::Integer(10),
            Token::RightParenthesis,
            Token::LeftBrace,
            Token::Return,
            Token::True,
            Token::Semicolon,
            Token::RightBrace,
            Token::Else,
            Token::LeftBrace,
            Token::Return,
            Token::False,
            Token::Semicolon,
            Token::RightBrace,

            Token::Integer(10),
            Token::Equals,
            Token::Integer(10),
            Token::Semicolon,

            Token::Integer(10),
            Token::NotEquals,
            Token::Integer(9),
            Token::Semicolon,
        ];

        let tokens = lexer.into_iter().collect::<Vec<Token>>();

        assert_eq!(expected_tokens, tokens);
    }

}
