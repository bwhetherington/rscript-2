use crate::parser::{Expression, ParseError, ParseResult, SpanData, Statement, Token};

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
        self.next_token().ok_or_else(|| ParseError::EOF)
    }

    fn try_parse_statement(&mut self) -> ParseResult<SpanData<Statement>> {
        todo!()
    }

    fn try_parse_expression(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_parse_number_expression()
            .or_else(|_| self.try_parse_identifier())
    }

    fn try_parse_number_expression(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_run(|parser| {
            let token = parser.parse_token()?;
            match token.value {
                Token::Number(n) => Ok(SpanData {
                    span: token.span,
                    value: Expression::Number(n),
                }),
                _ => Err(ParseError::custom("expected number")),
            }
        })
    }

    fn try_parse_identifier(&mut self) -> ParseResult<SpanData<Expression>> {
        self.try_run(|parser| {
            let token = parser.parse_token()?;
            match token.value {
                Token::Identifier(ident) => Ok(SpanData {
                    span: token.span,
                    value: Expression::Identifier(ident),
                }),
                _ => Err(ParseError::custom("expected number")),
            }
        })
    }
}
