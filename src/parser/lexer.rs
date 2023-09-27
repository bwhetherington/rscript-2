use crate::parser::{Point, Span, SpanData, Str};

#[derive(Debug)]
pub enum Token {
    Number(f64),
    Boolean(bool),
    String(Str),
    Identifier(Str),
    OpenBracket,
    CloseBracket,
    OpenBrace,
    CloseBrace,
    OpenParen,
    CloseParen,
    Period,
    Comma,
    Semicolon,
    Colon,
}

#[derive(Debug)]
pub enum LexError {
    EOF,
    Custom(Str),
}

impl LexError {
    pub fn custom(msg: impl Into<Str>) -> LexError {
        LexError::Custom(msg.into())
    }
}

pub type LexResult<T> = Result<T, LexError>;

pub struct Lexer {
    chars: Vec<char>,
    index: usize,
    name: Str,
    pos: Point,
}

fn is_atom(ch: char) -> bool {
    match ch {
        // Alphanumerics
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,

        _ => false,
    }
}

impl Lexer {
    pub fn new(name: impl Into<Str>, src: &str) -> Lexer {
        Lexer {
            chars: src.chars().collect(),
            index: 0,
            name: name.into(),
            pos: (0, 0).into(),
        }
    }

    fn get_char(&self) -> LexResult<char> {
        self.chars
            .get(self.index)
            .cloned()
            .ok_or_else(|| LexError::EOF)
    }

    fn next_char(&mut self) -> LexResult<char> {
        self.index += 1;
        let ch = self.get_char()?;

        match ch {
            '\n' => {
                self.pos.increment_row();
            }
            _ => {
                self.pos.increment_col();
            }
        }

        Ok(ch)
    }

    fn try_run<T>(&mut self, func: impl Fn(&mut Lexer) -> LexResult<T>) -> LexResult<T> {
        let start_index = self.index;

        let res = func(self);

        // Reset if there was an error
        if res.is_err() {
            self.index = start_index;
        }

        res
    }

    // fn try_parse_atom(&mut self) -> LexResult<SpanData<String>> {
    //     self.try_run(|lexer| {})
    // }

    fn try_parse_number(&mut self) -> LexResult<SpanData<f64>> {
        self.try_run(|lexer| {
            let mut string = String::new();
            let ch = lexer.next_char()?;
            Err(LexError::custom("foo"))
        })
    }

    pub fn next_token(&mut self) -> LexResult<SpanData<Token>> {
        Err(LexError::custom("no next token"))
    }
}
