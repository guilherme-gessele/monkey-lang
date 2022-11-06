use std::str::Chars;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
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
}

pub struct Lexer<'lexer> {
    input: Chars<'lexer>,
}

impl<'lexer> Lexer<'lexer> {
    pub fn new(input: &'lexer str) -> Self {
        let input = input.chars().into_iter();
        Lexer { 
            input
        }
    }
}

impl<'lexer> Iterator for Lexer<'lexer> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        
        let char = self.input.next();

        match char {
            Some('=') => Some(Token::Assign),
            Some('+') => Some(Token::Plus),
            Some('(') => Some(Token::LeftParenthesis),
            Some(')') => Some(Token::RightParenthesis),
            Some('{') => Some(Token::LeftBrace),
            Some('}') => Some(Token::RightBrace),
            Some(',') => Some(Token::Comma),
            Some(';') => Some(Token::Semicolon),
            Some(_) => None,
            None => None,
        }

    }
}

#[cfg(test)]
mod tests {

    use super::Lexer;
    use super::Token;

    #[test]
    fn next_token() {

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

}