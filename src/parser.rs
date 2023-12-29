use std::{iter::Peekable, vec};

use crate::lexer::Token;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ASTNode {
    ComprehensionSet(Box<ASTNode>, Box<ASTNode>),
    Correspondence(Box<ASTNode>, Box<ASTNode>),
    Equality(Box<ASTNode>, Box<ASTNode>),
    ExtensionSet(Vec<ASTNode>),
    Integer(String),
    Let(Box<ASTNode>, Vec<ASTNode>, Box<ASTNode>),
    Product(Box<ASTNode>, Box<ASTNode>),
    Sum(Box<ASTNode>, Box<ASTNode>),
    Symbol(String),
    Tuple(Vec<ASTNode>),
}

#[derive(Debug, PartialEq, Eq)]
enum ParserError {
    UnexpectedTokenError(Vec<Token>, Token),
    EOFError,
    EOFErrorExpecting(Vec<Token>),
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Precedence {
    Lowest,
    Comparison,
    Addition,
    Multiplication,
    Correspondence,
}

fn prec(tok: Token) -> Precedence {
    match tok {
        Token::Equals => Precedence::Comparison,
        Token::Plus => Precedence::Addition,
        Token::Times => Precedence::Multiplication,
        Token::Arrow => Precedence::Correspondence,
        _ => Precedence::Lowest,
    }
}

fn is_infix(tok: Token) -> bool {
    prec(tok) != Precedence::Lowest // yeah lgtm
}

struct Parser<T: Iterator<Item = Token>> {
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Iterator for Parser<T> {
    type Item = Result<ASTNode, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.tokens.peek() {
            None => None,
            Some(Token::Let) => Some(self.let_()),
            _ => Some(self.expression(Precedence::Lowest)),
        }
    }
}

impl <T: Iterator<Item = Token>> Parser<T> {
    fn let_(&mut self) -> Result<ASTNode, ParserError> {
        self.tokens.next();

        match (self.tokens.next(), self.tokens.next()) {
            (Some(Token::Ident(name)), Some(Token::Colon)) => self.type_().map(
                |tp| ASTNode::Let(Box::new(ASTNode::Symbol(name)), vec![], Box::new(tp))
            ),
            (Some(Token::Ident(name)), Some(Token::Assign)) => self.expression(Precedence::Lowest).map(
                |expr| ASTNode::Let(Box::new(ASTNode::Symbol(name)), vec![], Box::new(expr))
            ),
            (Some(Token::Ident(name)), Some(Token::Lparen)) => self.function_with_arguments(name),
            (Some(Token::Ident(_)), Some(tok)) => Err(ParserError::UnexpectedTokenError(vec![Token::Lparen, Token::Colon, Token::Assign], tok)),
            (Some(tok), _) => Err(ParserError::UnexpectedTokenError(vec![Token::Ident(String::from(""))], tok)),
            (None, _) => Err(ParserError::EOFErrorExpecting(vec![Token::Ident(String::from(""))])),
        }
    }

    fn function_with_arguments(&mut self, name: String) -> Result<ASTNode, ParserError> {
        let args_res = self.list(Token::Rparen, None);

        match (args_res, self.tokens.next()) {
            (Ok(args), Some(Token::Assign)) => match self.expression(Precedence::Lowest) {
                Ok(expr) => Ok(ASTNode::Let(Box::new(ASTNode::Symbol(name)), args, Box::new(expr))),
                err => err,
            },
            (Err(err), _) => Err(err),
            (_, Some(tok)) => Err(ParserError::UnexpectedTokenError(vec![Token::Assign], tok)),
            (Ok(_), None) => Err(ParserError::EOFErrorExpecting(vec![Token::Assign])),
        }
    }

    fn list(&mut self, terminator: Token, first: Option<ASTNode>) -> Result<Vec<ASTNode>, ParserError> {
        let mut res = match first {
            None => vec![],
            Some(node) => vec![node],
        };

        match self.tokens.peek() {
            Some(tok) if *tok == terminator => {
                self.tokens.next();   
                Ok(res)
            }
            None => Err(ParserError::EOFError),
            _ => loop { match self.expression(Precedence::Lowest) {
                Ok(expr) => {
                    res.push(expr);

                    match self.tokens.next() {
                        Some(Token::Comma) => continue,
                        Some(tok) if tok == terminator => break Ok(res),
                        Some(tok) => return Err(ParserError::UnexpectedTokenError(vec![Token::Comma, Token::Rparen], tok)),
                        None => return Err(ParserError::EOFErrorExpecting(vec![Token::Comma, Token::Rparen])),
                    }
                },
                Err(msg) => break Err(msg),
            }},
        }
    }

    fn expression(&mut self, precedence: Precedence) -> Result<ASTNode, ParserError> {
        let res = match self.tokens.next() {
            None => Err(ParserError::EOFError),
            Some(tok) => match tok {
                Token::Lparen => self.parenthesis(),
                Token::Lbrace => self.set(),
                Token::Integer(int) => Ok(ASTNode::Integer(int)),
                Token::Ident(literal) => Ok(ASTNode::Symbol(literal)),
                tok => Err(ParserError::UnexpectedTokenError(vec![Token::Lparen, Token::Lbrace, Token::Integer(String::from("")), Token::Ident(String::from(""))], tok)),
            },
        };

        match (res, self.tokens.next_if(|tok| is_infix(tok.clone()) && precedence < prec(tok.clone()))) {
            (Ok(lhs), Some(op_tok)) => self.infix(lhs, op_tok.clone(), prec(op_tok)),
            (res, _) => res,
        }
    }

    fn parenthesis(&mut self) -> Result<ASTNode, ParserError> {
        if self.tokens.next_if_eq(&Token::Rparen).is_some() {
            return Ok(ASTNode::Tuple(vec![]));
        }

        let res = self.expression(Precedence::Lowest);

        match self.tokens.next() {
            Some(Token::Rparen) => res,
            Some(tok) => Err(ParserError::UnexpectedTokenError(vec![Token::Rparen], tok)),
            None => Err(ParserError::EOFErrorExpecting(vec![Token::Rparen]))
        }
    }

    fn infix(&mut self, lhs: ASTNode, op: Token, precedence: Precedence) -> Result<ASTNode, ParserError> {
        let res = self.expression(precedence).map(
        |rhs| match op {
                Token::Plus => ASTNode::Sum(
                    Box::new(lhs),
                    Box::new(rhs)
                ),
                Token::Times => ASTNode::Product(
                    Box::new(lhs),
                    Box::new(rhs)
                ),
                Token::Arrow => ASTNode::Correspondence(
                    Box::new(lhs),
                    Box::new(rhs)
                ),
                Token::Equals => ASTNode::Equality(
                    Box::new(lhs), 
                    Box::new(rhs)
                ),
                _ => todo!(),
            }
        );

        match (res, self.tokens.next_if(|tok| is_infix(tok.clone()))) {
            (Ok(lhs), Some(op_tok)) => self.infix(lhs, op_tok.clone(), precedence),
            (res, _) => res,
        }
    }

    fn type_(&mut self) -> Result<ASTNode, ParserError> {
        self.expression(Precedence::Lowest)
    }

    fn set(&mut self) -> Result<ASTNode, ParserError> {
        if matches!(self.tokens.peek(), Some(&Token::Rbrace)) {
            return Ok(ASTNode::ExtensionSet(vec![]));
        }

        let first_res = self.expression(Precedence::Lowest);

        match (first_res, self.tokens.next()) {
            (Ok(first), Some(Token::Comma) | Some(Token::Rbrace)) => self.list(Token::Rbrace, Some(first))
                .map(|vec| ASTNode::ExtensionSet(vec)),
            (Ok(first), Some(Token::Colon)) => self.expression(Precedence::Lowest)
                .map(|second| ASTNode::ComprehensionSet(Box::new(first), Box::new(second))),
            (Ok(_), Some(tok)) => Err(ParserError::UnexpectedTokenError(vec![Token::Comma, Token::Rbrace, Token::Colon], tok)),
            (Ok(_), None) => Err(ParserError::EOFErrorExpecting(vec![Token::Comma, Token::Rbrace, Token::Colon])),
            (err, _) => err,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{iter, vec};
    use crate::lexer::Token;
    use super::*;

    macro_rules! token_iter {
        ($v:expr) => {
            $v.iter().map(|tok| tok.clone())
        };
    }

    fn parser_from<T: Iterator<Item = Token>>(tokens: T) -> Parser<T> {
        Parser { tokens: tokens.peekable() }
    }

    #[test]
    fn empty_expression() {
        assert_eq!(parser_from(iter::empty::<Token>()).next(), None);
    }

    #[test]
    fn integer() {
        let tokens = vec![Token::Integer(String::from("0"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(ASTNode::Integer(String::from("0"))))
        );
    }

    #[test]
    fn integer_in_parenthesis() {
        let tokens = vec![Token::Lparen, Token::Integer(String::from("365")), Token::Rparen];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(ASTNode::Integer(String::from("365"))))
        );
    }

    #[test]
    fn unbalanced_left_parenthesis() {
        let tokens = vec![Token::Lparen, Token::Integer(String::from("65"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Err(ParserError::EOFErrorExpecting(vec![Token::Rparen])))
        );
    }

    #[test]
    fn simple_sum() {
        let tokens = vec![Token::Integer(String::from("1")), Token::Plus, Token::Integer(String::from("1"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Sum(
                    Box::new(ASTNode::Integer(String::from("1"))),
                    Box::new(ASTNode::Integer(String::from("1")))
                )
            ))
        );
    }

    #[test]
    fn incomplete_sum() {
        let tokens = vec![Token::Integer(String::from("1")), Token::Plus];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Err(ParserError::EOFError))
        );
    }

    #[test]
    fn simple_product() {
        let tokens = vec![Token::Integer(String::from("1")), Token::Times, Token::Integer(String::from("1"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Product(
                    Box::new(ASTNode::Integer(String::from("1"))),
                    Box::new(ASTNode::Integer(String::from("1")))
                )
            ))
        );
    }

    #[test]
    fn product_and_sum() {
        let tokens = vec![Token::Integer(String::from("1")), Token::Times,
                                      Token::Integer(String::from("1")), Token::Plus, Token::Integer(String::from("1"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Sum(
                    Box::new(ASTNode::Product(
                        Box::new(ASTNode::Integer(String::from("1"))),
                        Box::new(ASTNode::Integer(String::from("1")))
                    )),
                    Box::new(ASTNode::Integer(String::from("1")))
                )
            )),
        );
    }

    #[test]
    fn let_statement() {
        let tokens = vec![Token::Let, Token::Ident(String::from('x')), Token::Assign, Token::Integer(String::from("1"))];
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Let(
                    Box::new(ASTNode::Symbol(String::from('x'))),
                    vec![],
                    Box::new(ASTNode::Integer(String::from("1")))
                )
            )),
        );
    }

    #[test]
    fn let_function_statement() {
        let tokens = vec![
            Token::Let, Token::Ident(String::from('f')), Token::Lparen ,Token::Ident(String::from('x')), Token::Comma, Token::Ident(String::from('y')), Token::Rparen,
            Token::Assign,
            Token::Ident(String::from('x')), Token::Plus, Token::Ident(String::from('y'))
        ];
        
        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Let(
                    Box::new(ASTNode::Symbol(String::from('f'))),
                    vec![ASTNode::Symbol(String::from('x')), ASTNode::Symbol(String::from('y'))],
                    Box::new(
                        ASTNode::Sum(
                            Box::new(ASTNode::Symbol(String::from('x'))),
                            Box::new(ASTNode::Symbol(String::from('y')))
                        )
                    )
                )
            )),
        );
    }

    #[test]
    fn let_function_signature() {
        let tokens = vec![
            Token::Let, Token::Ident(String::from('f')), Token::Colon,
            Token::Ident(String::from('a')), Token::Arrow,
            Token::Ident(String::from('a'))
        ];

        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::Let(
                    Box::new(ASTNode::Symbol(String::from('f'))),
                    vec![],
                    Box::new(
                        ASTNode::Correspondence(
                            Box::new(ASTNode::Symbol(String::from('a'))),
                            Box::new(ASTNode::Symbol(String::from('a')))
                        )
                    )
                )
            )),
        );
    }

    #[test]
    fn empty_set() {
        let tokens = vec![Token::Lbrace, Token::Rbrace];

        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(ASTNode::ExtensionSet(vec![])))
        );
    }

    #[test]
    fn set() {
        let tokens = vec![
            Token::Lbrace, Token::Lparen, Token::Integer(String::from("0")),
            Token::Rparen, Token::Comma, Token::Integer(String::from("0")),
            Token::Rbrace
        ];

        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::ExtensionSet(
                    vec![ASTNode::Integer(String::from("0")), ASTNode::Integer(String::from("0"))]
                )
            ))
        );
    }

    #[test]
    fn empty_tuple() {
        let tokens = vec![Token::Lparen, Token::Rparen];

        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(ASTNode::Tuple(vec![])))
        );
    }

    #[test]
    fn set_comprehension() {
        let tokens = vec![
            Token::Lbrace, Token::Ident(String::from("a")),
            Token::Colon, Token::Ident(String::from("a")),
            Token::Equals, Token::Integer(String::from("1"))
        ];

        assert_eq!(
            parser_from(token_iter!(tokens)).next(),
            Some(Ok(
                ASTNode::ComprehensionSet(
                    Box::new(ASTNode::Symbol(String::from("a"))),
                    Box::new(ASTNode::Equality(
                        Box::new(ASTNode::Symbol(String::from("a"))),
                        Box::new(ASTNode::Integer(String::from("1")))
                    ))
                )
            ))
        );
    }
}
