use std::{iter::Peekable, vec};

use crate::ast::*;
use crate::lexer::{Token, TokenType};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserError {
    ExpectedExpression(TokenType),
    UnexpectedToken(Vec<TokenType>, TokenType),
    EOFReached,
    EOFExpecting(Vec<TokenType>),
}

pub struct Parser<T: Iterator<Item = Token>> {
    tokens: Peekable<T>,
}

impl<T: Iterator<Item = Token>> Iterator for Parser<T> {
    type Item = Result<ASTNodeType, ParserError>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.peek_token() {
            None => None,
            _ => Some(self.expression(Precedence::Lowest)),
        }
    }
}

fn symbol(name: String) -> ASTNodeType {
    ASTNodeType::Symbol(name)
}

fn integer(int: String) -> ASTNodeType {
    ASTNodeType::Integer(int)
}

fn infix(op: InfixOperator, lhs: ASTNodeType, rhs: ASTNodeType) -> ASTNodeType {
    ASTNodeType::Infix(op, Box::new(lhs), Box::new(rhs))
}

fn prefix(op: PrefixOperator, val: ASTNodeType) -> ASTNodeType {
    ASTNodeType::Prefix(op, Box::new(val))
}

type NodeResult = Result<ASTNodeType, ParserError>;

impl<T: Iterator<Item = Token>> Parser<T> {
    pub fn program(&mut self) -> Vec<ASTNodeType> {
        let mut res = vec![];

        loop {
            let exp = self.next();

            match exp {
                Some(Ok(node)) => res.push(node),
                _ => break res,
            }
        }
    }

    fn next_token(&mut self) -> Option<TokenType> {
        match self.tokens.next() {
            Some(Token { token, .. }) => Some(token),
            _ => None,
        }
    }

    fn peek_token(&mut self) -> Option<TokenType> {
        match self.tokens.peek() {
            Some(Token { token, .. }) => Some(token.to_owned()),
            _ => None,
        }
    }

    fn let_(&mut self) -> NodeResult {
        let sg = self.signature()?;

        match self.next_token() {
            Some(TokenType::Assign) => self
                .expression(Precedence::Lowest)
                .map(|res| ASTNodeType::Let(Box::new(sg), vec![], Box::new(res))),
            _ => Ok(sg),
        }
    }

    fn signature(&mut self) -> NodeResult {
        match (self.next_token(), self.peek_token()) {
            (Some(TokenType::Ident(name)), Some(TokenType::Colon)) => {
                self.next_token();
                self.type_()
                    .map(|tp| ASTNodeType::Signature(Box::new(symbol(name)), Some(Box::new(tp))))
            }
            (Some(TokenType::Ident(name)), Some(TokenType::Lparen)) => {
                self.next_token();
                self.let_function_with_arguments(name)
            }
            (Some(TokenType::Ident(name)), _) => {
                Ok(ASTNodeType::Signature(Box::new(symbol(name)), None))
            }
            (Some(tok), _) => Err(ParserError::UnexpectedToken(
                vec![TokenType::Ident(String::from(""))],
                tok,
            )),
            (None, _) => Err(ParserError::EOFExpecting(vec![TokenType::Ident(
                String::from(""),
            )])),
        }
    }

    fn let_function_with_arguments(&mut self, name: String) -> NodeResult {
        let args_res = self.list(TokenType::Rparen, None);

        match (args_res, self.next_token()) {
            (Ok(args), Some(TokenType::Assign)) => match self.expression(Precedence::Lowest) {
                Ok(expr) => Ok(ASTNodeType::Let(
                    Box::new(symbol(name)),
                    args,
                    Box::new(expr),
                )),
                err => err,
            },
            (Err(err), _) => Err(err),
            (_, Some(tok)) => Err(ParserError::UnexpectedToken(vec![TokenType::Assign], tok)),
            (Ok(_), None) => Err(ParserError::EOFExpecting(vec![TokenType::Assign])),
        }
    }

    fn list(
        &mut self,
        terminator: TokenType,
        first: Option<ASTNodeType>,
    ) -> Result<Vec<ASTNodeType>, ParserError> {
        let mut res = match first {
            None => vec![],
            Some(node) => vec![node],
        };

        match self.peek_token() {
            Some(tok) if tok == terminator => {
                self.next_token();
                Ok(res)
            }
            None => Err(ParserError::EOFReached),
            _ => loop {
                let expr = self.expression(Precedence::Lowest)?;
                res.push(expr);

                match self.next_token() {
                    Some(TokenType::Comma) => continue,
                    Some(tok) if tok == terminator => break Ok(res),
                    Some(tok) => {
                        break Err(ParserError::UnexpectedToken(
                            vec![TokenType::Comma, terminator],
                            tok,
                        ))
                    }
                    None => {
                        break Err(ParserError::EOFExpecting(vec![
                            TokenType::Comma,
                            terminator,
                        ]))
                    }
                }
            },
        }
    }

    fn expression(&mut self, precedence: Precedence) -> NodeResult {
        let mut expr = match self.next_token() {
            None => Err(ParserError::EOFReached),
            Some(tok) => match tok {
                TokenType::Char(chr) => Ok(ASTNodeType::Char(chr)),
                TokenType::For => self.for_(),
                TokenType::If => self.if_(),
                TokenType::Let => self.let_(),
                TokenType::True => Ok(ASTNodeType::Boolean(true)),
                TokenType::False => Ok(ASTNodeType::Boolean(false)),
                TokenType::Lparen => self.parenthesis(),
                TokenType::Lbrace => self.set(),
                TokenType::Lbrack => self.my_list(),
                TokenType::Integer(int) => Ok(integer(int)),
                TokenType::Ident(literal) => Ok(symbol(literal)),
                TokenType::String(str) => Ok(ASTNodeType::String(str)),
                TokenType::Wildcard => Ok(ASTNodeType::Wildcard),
                tok if PrefixOperator::from(tok.clone()).is_some() => {
                    self.prefix(PrefixOperator::from(tok).unwrap())
                }
                tok => Err(ParserError::ExpectedExpression(tok)),
            },
        }?;

        while let Some(op) = self.current_infix() {
            if precedence < op.precedence() {
                self.next_token();
                expr = self.infix(expr, op)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    fn my_list(&mut self) -> NodeResult {
        if matches!(self.peek_token(), Some(TokenType::Rbrack)) {
            self.next_token();
            return Ok(ASTNodeType::ExtensionList(vec![]));
        }

        let first = self.expression(Precedence::Lowest)?;

        match self.next_token() {
            Some(TokenType::Colon) => self.comprehension_list(first),
            Some(TokenType::Comma) => self
                .list(TokenType::Rbrack, Some(first))
                .map(ASTNodeType::ExtensionList),
            Some(TokenType::Rbrack) => Ok(ASTNodeType::ExtensionList(vec![first])),
            Some(TokenType::VerticalBar) => self.prepend(first),
            Some(tok) => Err(ParserError::UnexpectedToken(
                vec![TokenType::Colon, TokenType::Comma, TokenType::Rbrack],
                tok,
            )),
            None => Err(ParserError::EOFExpecting(vec![
                TokenType::Colon,
                TokenType::Comma,
                TokenType::Comma,
                TokenType::Rbrack,
            ])),
        }
    }

    fn comprehension_list(&mut self, first: ASTNodeType) -> NodeResult {
        let last = self.expression(Precedence::Lowest)?;
        self.consume(TokenType::Rbrack)?;

        Ok(ASTNodeType::ComprehensionList(
            Box::new(first),
            Box::new(last),
        ))
    }

    fn prepend(&mut self, first: ASTNodeType) -> NodeResult {
        let last = self.expression(Precedence::Lowest)?;

        self.consume(TokenType::Rbrack)?;

        Ok(ASTNodeType::Prepend(Box::new(first), Box::new(last)))
    }

    fn for_(&mut self) -> NodeResult {
        let ident = match self.next_token() {
            Some(TokenType::Ident(s)) => Ok(s),
            Some(tok) => Err(ParserError::UnexpectedToken(
                vec![TokenType::Ident(String::from(""))],
                tok,
            )),
            None => Err(ParserError::EOFExpecting(vec![TokenType::Ident(
                String::from(""),
            )])),
        }?;

        self.consume(TokenType::In)?;

        let iter = self.expression(Precedence::Lowest)?;

        self.consume(TokenType::Colon)?;

        let proc = match self.expression(Precedence::Lowest)? {
            ASTNodeType::Tuple(v) => v,
            node => vec![node],
        };

        Ok(ASTNodeType::For(ident, Box::new(iter), proc))
    }

    fn consume(&mut self, expected_tok: TokenType) -> Result<(), ParserError> {
        match self.next_token() {
            Some(tok) if tok == expected_tok => Ok(()),
            Some(tok) => Err(ParserError::UnexpectedToken(vec![expected_tok], tok)),
            None => Err(ParserError::EOFExpecting(vec![expected_tok])),
        }
    }

    fn if_(&mut self) -> NodeResult {
        let cond = self.expression(Precedence::Lowest)?;

        self.consume(TokenType::Then)?;

        let first_res = self.expression(Precedence::Lowest)?;

        self.consume(TokenType::Else)?;

        let second_res = self.expression(Precedence::Lowest)?;

        Ok(ASTNodeType::If(
            Box::new(cond),
            Box::new(first_res),
            Box::new(second_res),
        ))
    }

    fn current_infix(&mut self) -> Option<InfixOperator> {
        self.peek_token().and_then(InfixOperator::from)
    }

    fn prefix(&mut self, op: PrefixOperator) -> NodeResult {
        self.expression(Precedence::Highest)
            .map(|expr| prefix(op, expr))
    }

    fn parenthesis(&mut self) -> NodeResult {
        if matches!(self.peek_token(), Some(TokenType::Rparen)) {
            self.next_token();
            return Ok(ASTNodeType::Tuple(vec![]));
        }

        let res = self.expression(Precedence::Lowest)?;

        match self.next_token() {
            Some(TokenType::Rparen) => Ok(res),
            Some(TokenType::Comma) => self
                .list(TokenType::Rparen, Some(res))
                .map(ASTNodeType::Tuple),
            Some(tok) => Err(ParserError::UnexpectedToken(vec![TokenType::Rparen], tok)),
            None => Err(ParserError::EOFExpecting(vec![TokenType::Rparen])),
        }
    }

    fn infix(&mut self, lhs: ASTNodeType, op: InfixOperator) -> NodeResult {
        if op == InfixOperator::Call {
            self.list(TokenType::Rparen, None)
                .map(|args| infix(op, lhs, ASTNodeType::Tuple(args)))
        } else {
            self.expression(op.precedence())
                .map(|rhs| infix(op, lhs, rhs))
        }
    }

    fn type_(&mut self) -> NodeResult {
        self.expression(Precedence::Lowest)
    }

    fn set(&mut self) -> NodeResult {
        if matches!(self.peek_token(), Some(TokenType::Rbrace)) {
            self.next_token();
            return Ok(ASTNodeType::ExtensionSet(vec![]));
        }

        let first = self.expression(Precedence::Lowest)?;

        match self.next_token() {
            Some(TokenType::Comma) => self
                .list(TokenType::Rbrace, Some(first))
                .map(ASTNodeType::ExtensionSet),
            Some(TokenType::Colon) => self
                .expression(Precedence::Lowest)
                .map(|second| ASTNodeType::ComprehensionSet(Box::new(first), Box::new(second))),
            Some(TokenType::Rbrace) => Ok(ASTNodeType::ExtensionSet(vec![first])),
            Some(tok) => Err(ParserError::UnexpectedToken(
                vec![TokenType::Comma, TokenType::Rbrace, TokenType::Colon],
                tok,
            )),
            None => Err(ParserError::EOFExpecting(vec![
                TokenType::Comma,
                TokenType::Rbrace,
                TokenType::Colon,
            ])),
        }
    }
}

pub fn parser_from<T: Iterator<Item = Token>>(tokens: T) -> Parser<T> {
    Parser {
        tokens: tokens.peekable(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::Position, lexer::build_lexer, parser::integer};
    use std::iter;

    fn iter_from(v: Vec<TokenType>) -> impl Iterator<Item = Token> {
        v.into_iter()
            .map(|tok| Token::new(tok, Position::new(0, 0)))
    }

    #[test]
    fn empty_expression() {
        assert_eq!(parser_from(iter::empty::<Token>()).next(), None);
    }

    #[test]
    fn integer_alone() {
        let tokens = vec![TokenType::Integer(String::from("0"))];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(integer(String::from("0"))))
        );
    }

    #[test]
    fn integer_in_parenthesis() {
        let tokens = vec![
            TokenType::Lparen,
            TokenType::Integer(String::from("365")),
            TokenType::Rparen,
        ];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(integer(String::from("365"))))
        );
    }

    #[test]
    fn unbalanced_left_parenthesis() {
        let tokens = vec![TokenType::Lparen, TokenType::Integer(String::from("65"))];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Err(ParserError::EOFExpecting(vec![TokenType::Rparen])))
        );
    }

    #[test]
    fn simple_sum() {
        let tokens = vec![
            TokenType::Integer(String::from("1")),
            TokenType::Plus,
            TokenType::Integer(String::from("1")),
        ];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(infix(
                InfixOperator::Sum,
                integer(String::from("1")),
                integer(String::from("1"))
            )))
        );
    }

    #[test]
    fn incomplete_sum() {
        let tokens = vec![TokenType::Integer(String::from("1")), TokenType::Plus];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Err(ParserError::EOFReached))
        );
    }

    #[test]
    fn product_and_power() {
        let tokens = vec![
            TokenType::Integer(String::from("1")),
            TokenType::Times,
            TokenType::Integer(String::from("2")),
            TokenType::ToThe,
            TokenType::Integer(String::from("2")),
        ];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(infix(
                InfixOperator::Product,
                integer(String::from("1")),
                infix(
                    InfixOperator::Exponentiation,
                    integer(String::from("2")),
                    integer(String::from("2")),
                )
            )))
        );
    }

    #[test]
    fn division_and_sum() {
        let tokens = vec![
            TokenType::Integer(String::from("1")),
            TokenType::Over,
            TokenType::Integer(String::from("1")),
            TokenType::Plus,
            TokenType::Integer(String::from("1")),
        ];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(infix(
                InfixOperator::Sum,
                infix(
                    InfixOperator::Division,
                    integer(String::from("1")),
                    integer(String::from("1"))
                ),
                integer(String::from("1"))
            ))),
        );
    }

    #[test]
    fn let_statement() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Ident(String::from('x')),
            TokenType::Assign,
            TokenType::Integer(String::from("1")),
        ];
        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Let(
                Box::new(ASTNodeType::Signature(
                    Box::new(symbol(String::from('x'))),
                    None
                )),
                vec![],
                Box::new(integer(String::from("1")))
            ))),
        );
    }

    #[test]
    fn comparison_precedence() {
        let tokens = vec![
            TokenType::Integer(String::from("1")),
            TokenType::Plus,
            TokenType::Integer(String::from("5")),
            TokenType::NotEqual,
            TokenType::Integer(String::from("6")),
            TokenType::Mod,
            TokenType::Integer(String::from("2")),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(infix(
                InfixOperator::NotEquality,
                infix(
                    InfixOperator::Sum,
                    integer(String::from("1")),
                    integer(String::from("5")),
                ),
                infix(
                    InfixOperator::Mod,
                    integer(String::from("6")),
                    integer(String::from("2")),
                )
            )))
        );
    }

    #[test]
    fn let_function_statement() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Ident(String::from('f')),
            TokenType::Lparen,
            TokenType::Ident(String::from('x')),
            TokenType::Comma,
            TokenType::Ident(String::from('y')),
            TokenType::Rparen,
            TokenType::Assign,
            TokenType::Ident(String::from('x')),
            TokenType::Plus,
            TokenType::Ident(String::from('y')),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Let(
                Box::new(symbol(String::from('f'))),
                vec![symbol(String::from('x')), symbol(String::from('y'))],
                Box::new(infix(
                    InfixOperator::Sum,
                    symbol(String::from('x')),
                    symbol(String::from('y'))
                ))
            ))),
        );
    }

    #[test]
    fn let_function_signature() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Ident(String::from('f')),
            TokenType::Colon,
            TokenType::Ident(String::from('a')),
            TokenType::Arrow,
            TokenType::Ident(String::from('a')),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Signature(
                Box::new(symbol(String::from('f'))),
                Some(Box::new(infix(
                    InfixOperator::Correspondence,
                    symbol(String::from('a')),
                    symbol(String::from('a'))
                )))
            ))),
        );
    }

    #[test]
    fn empty_set() {
        let tokens = vec![TokenType::Lbrace, TokenType::Rbrace];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::ExtensionSet(vec![])))
        );
    }

    #[test]
    fn set() {
        let tokens = vec![
            TokenType::Lbrace,
            TokenType::Lparen,
            TokenType::True,
            TokenType::Rparen,
            TokenType::Comma,
            TokenType::False,
            TokenType::Rbrace,
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::ExtensionSet(vec![
                ASTNodeType::Boolean(true),
                ASTNodeType::Boolean(false)
            ])))
        );
    }

    #[test]
    fn empty_tuple() {
        let tokens = vec![TokenType::Lparen, TokenType::Rparen];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Tuple(vec![])))
        );
    }

    #[test]
    fn tuple() {
        let tokens = vec![
            TokenType::Lparen,
            TokenType::Ident(String::from("Real")),
            TokenType::Comma,
            TokenType::Ident(String::from("Real")),
            TokenType::Rparen,
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Tuple(vec![
                symbol(String::from("Real")),
                symbol(String::from("Real"))
            ])))
        );
    }

    #[test]
    fn set_comprehension() {
        let tokens = vec![
            TokenType::Lbrace,
            TokenType::Ident(String::from("a")),
            TokenType::Colon,
            TokenType::Ident(String::from("a")),
            TokenType::Equals,
            TokenType::Integer(String::from("1")),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::ComprehensionSet(
                Box::new(symbol(String::from("a"))),
                Box::new(infix(
                    InfixOperator::Equality,
                    symbol(String::from("a")),
                    integer(String::from("1"))
                ))
            )))
        );
    }

    #[test]
    fn typed_let() {
        let tokens = vec![
            TokenType::Let,
            TokenType::Ident(String::from("x")),
            TokenType::Colon,
            TokenType::Ident(String::from("Real")),
            TokenType::Assign,
            TokenType::Integer(String::from("1")),
            TokenType::Plus,
            TokenType::Integer(String::from("0")),
            TokenType::Mod,
            TokenType::Integer(String::from("2")),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::Let(
                Box::new(ASTNodeType::Signature(
                    Box::new(symbol(String::from("x"))),
                    Some(Box::new(symbol(String::from("Real"))))
                )),
                vec![],
                Box::new(infix(
                    InfixOperator::Sum,
                    integer(String::from("1")),
                    infix(
                        InfixOperator::Mod,
                        integer(String::from("0")),
                        integer(String::from("2"))
                    )
                ))
            )))
        );
    }

    #[test]
    fn shift_operator() {
        let tokens = vec![
            TokenType::Ident(String::from('x')),
            TokenType::Minus,
            TokenType::Integer(String::from('1')),
            TokenType::LeftShift,
            TokenType::Integer(String::from('1')),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(infix(
                InfixOperator::LeftShift,
                infix(
                    InfixOperator::Substraction,
                    symbol(String::from('x')),
                    integer(String::from('1'))
                ),
                integer(String::from('1'))
            )))
        );
    }

    #[test]
    fn shift_and_comparison() {
        let lexer = build_lexer("1 << 1 > 1").map(|res| res.unwrap());

        assert_eq!(
            parser_from(lexer).next(),
            Some(Ok(infix(
                InfixOperator::Greater,
                infix(
                    InfixOperator::LeftShift,
                    integer(String::from('1')),
                    integer(String::from('1'))
                ),
                integer(String::from('1'))
            )))
        );
    }

    #[test]
    fn bitwise() {
        let lexer = build_lexer("a & b || c").map(|res| res.unwrap());

        assert_eq!(
            parser_from(lexer).next(),
            Some(Ok(infix(
                InfixOperator::Or,
                infix(
                    InfixOperator::BitwiseAnd,
                    symbol(String::from('a')),
                    symbol(String::from('b'))
                ),
                symbol(String::from('c'))
            )))
        );
    }

    #[test]
    fn logic_infix_operators() {
        let lexer = build_lexer("a && b || c");

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Or,
                infix(
                    InfixOperator::LogicAnd,
                    symbol(String::from('a')),
                    symbol(String::from('b'))
                ),
                symbol(String::from('c'))
            )))
        );
    }

    #[test]
    fn complex_precedence() {
        let lexer = build_lexer("a + b || a & b << c");

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Or,
                infix(
                    InfixOperator::Sum,
                    symbol(String::from('a')),
                    symbol(String::from('b'))
                ),
                infix(
                    InfixOperator::BitwiseAnd,
                    symbol(String::from('a')),
                    infix(
                        InfixOperator::LeftShift,
                        symbol(String::from('b')),
                        symbol(String::from('c'))
                    )
                )
            )))
        );
    }

    #[test]
    fn bitwise_xor() {
        let lexer = build_lexer("a ^ b & c || d");

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Or,
                infix(
                    InfixOperator::BitwiseXor,
                    symbol(String::from('a')),
                    infix(
                        InfixOperator::BitwiseAnd,
                        symbol(String::from('b')),
                        symbol(String::from('c'))
                    )
                ),
                symbol(String::from('d'))
            )))
        );
    }

    #[test]
    fn something_after_empty_set() {
        let lexer = build_lexer("({}, 0)");

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::Tuple(vec![
                ASTNodeType::ExtensionSet(vec![]),
                integer(String::from('0'))
            ])))
        );
    }

    #[test]
    fn prefixes() {
        let lexer = build_lexer("!(~1 /= -1)");

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(prefix(
                PrefixOperator::LogicNot,
                infix(
                    InfixOperator::NotEquality,
                    prefix(
                        PrefixOperator::BitwiseNot,
                        integer(String::from("1")),
                    ),
                    prefix(
                        PrefixOperator::Minus,
                        integer(String::from("1")),
                    ),
                )
            )))
        );
    }

    #[test]
    fn if_expr() {
        let tokens = vec![
            TokenType::If,
            TokenType::Ident(String::from("a")),
            TokenType::Less,
            TokenType::Integer(String::from("0")),
            TokenType::Then,
            TokenType::Minus,
            TokenType::Ident(String::from("a")),
            TokenType::Else,
            TokenType::Ident(String::from("a")),
        ];

        assert_eq!(
            parser_from(iter_from(tokens)).next(),
            Some(Ok(ASTNodeType::If(
                Box::new(infix(
                    InfixOperator::Less,
                    symbol(String::from("a")),
                    integer(String::from("0"))
                )),
                Box::new(prefix(
                    PrefixOperator::Minus,
                    symbol(String::from("a"))
                )),
                Box::new(symbol(String::from("a"))),
            )))
        );
    }

    #[test]
    fn program() {
        let input = "let a := 5
        a * a
        ";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).program(),
            vec![
                ASTNodeType::Let(
                    Box::new(ASTNodeType::Signature(
                        Box::new(symbol(String::from("a"))),
                        None,
                    )),
                    vec![],
                    Box::new(integer(String::from("5"))),
                ),
                infix(
                    InfixOperator::Product,
                    symbol(String::from("a")),
                    symbol(String::from("a")),
                )
            ],
        );
    }

    #[test]
    fn function_call() {
        let input = "f(x, y)";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Call,
                symbol(String::from("f")),
                ASTNodeType::Tuple(vec![symbol(String::from("x")), symbol(String::from("y")),]),
            ))),
        );
    }

    #[test]
    fn comma_last_item() {
        let input = "(1,)";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::Tuple(vec![integer(String::from("1"))]))),
        );
    }

    #[test]
    fn anon_function() {
        let input = "x -> 2*x";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Correspondence,
                symbol(String::from("x")),
                infix(
                    InfixOperator::Product,
                    integer(String::from("2")),
                    symbol(String::from("x")),
                )
            )))
        );
    }

    #[test]
    fn anon_function_call() {
        let input = "((x, y) -> x)(1, 2)";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::Call,
                infix(
                    InfixOperator::Correspondence,
                    ASTNodeType::Tuple(vec![symbol(String::from("x")), symbol(String::from("y")),]),
                    symbol(String::from("x")),
                ),
                ASTNodeType::Tuple(vec![integer(String::from("1")), integer(String::from("2")),]),
            )))
        );
    }

    #[test]
    fn char_and_string() {
        let input = "('a', \"b\")";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::Tuple(vec![
                ASTNodeType::Char('a'),
                ASTNodeType::String(String::from('b')),
            ])))
        );
    }

    #[test]
    fn for_loop() {
        let input = "for i in list: println(i)";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::For(
                String::from("i"),
                Box::new(symbol(String::from("list"))),
                vec![infix(
                    InfixOperator::Call,
                    symbol(String::from("println")),
                    ASTNodeType::Tuple(vec![symbol(String::from("i"))]),
                )]
            )))
        );
    }

    #[test]
    fn in_question() {
        let input = "1 in { k : k >= 1 }";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(infix(
                InfixOperator::In,
                integer(String::from("1")),
                ASTNodeType::ComprehensionSet(
                    Box::new(symbol(String::from("k"))),
                    Box::new(infix(
                        InfixOperator::GreaterEqual,
                        symbol(String::from("k")),
                        integer(String::from("1")),
                    )),
                ),
            )))
        );
    }

    #[test]
    fn list() {
        let input = "[[], 2]";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::ExtensionList(vec![
                ASTNodeType::ExtensionList(vec![]),
                integer(String::from("2")),
            ]),)),
        );
    }

    #[test]
    fn comprehension_list() {
        let input = "[ k in [1, 2] : k - 1 = 0 ]";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::ComprehensionList(
                Box::new(infix(
                    InfixOperator::In,
                    symbol(String::from("k")),
                    ASTNodeType::ExtensionList(vec![
                        integer(String::from("1")),
                        integer(String::from("2")),
                    ]),
                )),
                Box::new(infix(
                    InfixOperator::Equality,
                    infix(
                        InfixOperator::Substraction,
                        symbol(String::from("k")),
                        integer(String::from("1")),
                    ),
                    integer(String::from("0")),
                )),
            )))
        );
    }

    #[test]
    fn singleton_empty_set() {
        let input = "{{}}";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::ExtensionSet(vec![
                ASTNodeType::ExtensionSet(vec![])
            ]))),
        );
    }

    #[test]
    fn wildcard() {
        let input = "[a, 1, _]";

        let lexer = build_lexer(input);

        assert_eq!(
            parser_from(lexer.map(|res| res.unwrap())).next(),
            Some(Ok(ASTNodeType::ExtensionList(vec![
                symbol(String::from("a")),
                integer(String::from("1")),
                ASTNodeType::Wildcard,
            ]),)),
        );
    }

    #[test]
    fn prepend() {
        let code = "[1|[2,3]]";
        let lexer = build_lexer(code).map(|res| res.unwrap());

        assert_eq!(
            parser_from(lexer).next(),
            Some(Ok(ASTNodeType::Prepend(
                Box::new(integer(String::from("1"))),
                Box::new(ASTNodeType::ExtensionList(vec![
                    integer(String::from("2")),
                    integer(String::from("3")),
                ])),
            ))),
        );
    }

    #[test]
    fn consume_comprehension_list() {
        let input = "[a : a in b] + []";
        let lexer = build_lexer(input).map(|res| res.unwrap());

        assert_eq!(
            parser_from(lexer).next(),
            Some(Ok(infix(
                InfixOperator::Sum,
                ASTNodeType::ComprehensionList(
                    Box::new(symbol(String::from("a"))),
                    Box::new(infix(
                        InfixOperator::In,
                        symbol(String::from("a")),
                        symbol(String::from("b")),
                    ))
                ),
                ASTNodeType::ExtensionList(vec![]),
            )))
        );
    }
}
