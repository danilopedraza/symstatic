use std::{str::Chars, iter::Peekable};

#[derive(PartialEq, Eq, Debug)]
enum Token {
    PLUS,
    IDENT(String),
}

struct Lexer<'a> {
    input: Peekable<Chars<'a>>,
}

impl Iterator for Lexer<'_> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        match self.input.peek() {
            None => None,
            Some(' ') | Some('\t') => self.whitespace(),
            Some(chr) => Some(match chr {
                '+' => self.plus(),
                _ => self.identifier(),
            }),
        }
    }
}

impl Lexer<'_> {
    fn whitespace(&mut self) -> Option<Token> {
        self.input.next();
        self.next()
    }

    fn plus(&mut self) -> Token {
        self.input.next();
        Token::PLUS
    }

    fn identifier(&mut self) -> Token {
        Token::IDENT(String::from(self.input.next().unwrap()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn lexer_from(string: &str) -> Lexer {
        Lexer { input: string.chars().peekable() }
    }

    #[test]
    fn empty_string() {
        assert!(lexer_from("").next().is_none());
    }

    #[test]
    fn plus_operator() {
        assert_eq!(lexer_from("+").next(), Some(Token::PLUS));
    }

    #[test]
    fn whitespace() {
        assert_eq!(lexer_from(" \t").next(), None);
    }

    #[test]
    fn simple_expression() {
        assert_eq!(
            lexer_from("x + y").collect::<Vec<_>>(),
            vec![Token::IDENT(String::from('x')), Token::PLUS, Token::IDENT(String::from('y'))],
        );
    }
}
