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

pub struct Lexer {}

impl Lexer {
    pub fn new() -> Self {
        Lexer {}
    }

    pub fn parse(&self, input: &str) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        let mut iter = input.chars().into_iter().peekable();

        while let Some(c) = iter.next() {
            let token = match c {
                ASSIGN => {
                    if let Some(p) = iter.peek() {
                        if p == &ASSIGN {
                            iter.next();
                            Token::Equals
                        } else {
                            Token::Assign
                        }
                    } else {
                        Token::Illegal
                    }
                }
                PLUS => Token::Plus,
                MINUS => Token::Minus,
                BANG => {
                    if let Some(p) = iter.peek() {
                        if p == &ASSIGN {
                            iter.next();
                            Token::NotEquals
                        } else {
                            Token::Bang
                        }
                    } else {
                        Token::Illegal
                    }
                }
                SLASH => Token::Slash,
                ASTERISK => Token::Asterisk,
                LESS_THAN => Token::LessThan,
                GREATER_THAN => Token::GreatherThan,

                LEFT_PARENTHESIS => Token::LeftParenthesis,
                RIGHT_PARENTHESIS => Token::RightParenthesis,
                LEFT_BRACE => Token::LeftBrace,
                RIGHT_BRACE => Token::RightBrace,
                COMMA => Token::Comma,
                SEMICOLON => Token::Semicolon,
                c if c.is_digit(10) => {
                    let mut output = vec![c];

                    while let Some(c) = iter.next_if(|c| c.is_digit(10)) {
                        output.push(c);
                    }

                    let str = String::from_iter(output);

                    Token::Integer(str.parse::<usize>().unwrap())
                }
                c if c.is_ascii_alphabetic() => {
                    let mut output = vec![c];

                    while let Some(c) = iter.next_if(|c| c.is_ascii_alphabetic()) {
                        output.push(c);
                    }

                    let str = String::from_iter(output);

                    match str.as_str() {
                        LET => Token::Let,
                        FUNCTION => Token::Function,
                        TRUE => Token::True,
                        FALSE => Token::False,
                        IF => Token::If,
                        ELSE => Token::Else,
                        RETURN => Token::Return,
                        _ => Token::Identifier(str),
                    }
                }
                c if c.is_whitespace() => continue,
                _ => Token::Illegal,
            };

            tokens.push(token);
        }

        tokens
    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use super::Token;

    #[test]
    fn delimiters() {
        let input = "=+(){},;";
        let lexer = Lexer::new();

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

        let tokens = lexer.parse(input);

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

        let lexer = Lexer::new();

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

        let tokens = lexer.parse(input);

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

        let lexer = Lexer::new();

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

        let tokens = lexer.parse(input);

        assert_eq!(expected_tokens, tokens);
    }
}
