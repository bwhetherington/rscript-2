use crate::parser::{Expression, ParseError, ParseResult, Span, SpanData, Statement, Token};

pub struct AstParser {
    tokens: Vec<SpanData<Token>>,
    index: usize,
}

impl AstParser {
    pub fn new(tokens: Vec<SpanData<Token>>) -> AstParser {
        AstParser { tokens, index: 0 }
    }

    fn try_run<T>(&mut self, parse: impl Fn(&mut Self) -> ParseResult<T>) -> ParseResult<T> {
        let start = self.index;
        match parse(self) {
            ok @ Ok(_) => ok,
            err @ Err(_) => {
                self.index = start;
                err
            }
        }
    }

    fn get_token(&self) -> Option<&SpanData<Token>> {
        self.tokens.get(self.index)
    }

    fn next_token(&mut self) -> Option<SpanData<Token>> {
        let token = self.get_token().cloned();
        self.index += 1;
        token
    }

    fn parse_token(&mut self) -> ParseResult<SpanData<Token>> {
        self.next_token().ok_or_else(|| ParseError::ExpectedToken)
    }

    fn try_parse_token(
        &mut self,
        pred: impl Fn(&Token) -> bool,
        why: &'static str,
    ) -> ParseResult<SpanData<Token>> {
        let token = self.parse_token()?;
        if pred(&token.value) {
            Ok(token)
        } else {
            Err(ParseError::custom(why))
        }
    }

    fn try_parse_statement(&mut self) -> ParseResult<SpanData<Statement>> {
        todo!()
    }

    fn try_parse_expression(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_parse_token_expression()
    }

    fn try_parse_parentheses(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_run(|parser| {
            let start = parser.try_parse_token(
                |token| matches!(token, Token::OpenParen),
                "expected open paren",
            )?;
            let inner = parser.try_parse_expression()?;
            let stop = parser.try_parse_token(
                |token| matches!(token, Token::CloseParen),
                "expected close paren",
            )?;
            Ok(SpanData {
                span: start.span.to(&stop.span),
                value: inner.value,
            })
        })
    }

    fn try_parse_token_expression(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_run(|parser| {
            let token = parser.parse_token()?;
            let expr = match token.value {
                Token::Boolean(b) => Ok(Expression::Boolean(b)),
                Token::Number(n) => Ok(Expression::Number(n)),
                Token::String(s) => Ok(Expression::String(s)),
                Token::Identifier(i) => Ok(Expression::Identifier(i)),
                Token::None => Ok(Expression::None),
                _ => Err(ParseError::ExpectedToken),
            }?;
            Ok(SpanData {
                span: token.span,
                value: expr,
            })
        })
    }
}
