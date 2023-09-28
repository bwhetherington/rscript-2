use std::{collections::HashSet, sync::OnceLock};

use crate::parser::{Point, PrefixTree, Span, SpanData, Str};

#[derive(Clone, Debug)]
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
    Equals,
    DoubleEquals,
}

static SYMBOL_TREE: OnceLock<PrefixTree<Token>> = OnceLock::new();

pub fn get_symbol_tree() -> &'static PrefixTree<Token> {
    SYMBOL_TREE.get_or_init(|| {
        PrefixTree::from_iter([
            (".", Token::Period),
            (",", Token::Comma),
            (";", Token::Semicolon),
            (":", Token::Colon),
            ("(", Token::OpenParen),
            (")", Token::CloseParen),
            ("[", Token::OpenBracket),
            ("]", Token::CloseBracket),
            ("{", Token::OpenBrace),
            ("}", Token::CloseBrace),
            ("=", Token::Equals),
            ("==", Token::DoubleEquals),
        ])
    })
}

static SYMBOL_CHARS: OnceLock<HashSet<char>> = OnceLock::new();

fn get_symbol_chars() -> &'static HashSet<char> {
    SYMBOL_CHARS.get_or_init(|| get_symbol_tree().get_all_chars())
}

#[derive(Debug)]
pub enum LexError {
    EOF,
    UnknownSymbol(Str),
    Custom(Str),
}

impl LexError {
    pub fn unknown_symbol(msg: impl Into<Str>) -> LexError {
        LexError::UnknownSymbol(msg.into())
    }

    pub fn custom(msg: impl Into<Str>) -> LexError {
        LexError::Custom(msg.into())
    }
}

pub type LexResult<T> = Result<T, LexError>;

pub struct Lexer {
    lines: Vec<Vec<char>>,
    pos: Point,
    name: Str,
}

fn is_atom(ch: char) -> bool {
    match ch {
        // Alphanumerics
        'a'..='z' | 'A'..='Z' | '0'..='9' => true,

        _ => false,
    }
}

pub fn split_lines(src: &str) -> Vec<Vec<char>> {
    let mut lines = Vec::new();
    let mut line = Vec::new();
    for ch in src.chars() {
        line.push(ch);
        if ch == '\n' {
            lines.push(line);
            line = Vec::new();
        }
    }
    lines.push(line);
    lines
}

impl Lexer {
    pub fn new(name: impl Into<Str>, src: &str) -> Lexer {
        Lexer {
            lines: split_lines(src),
            pos: (0, 0).into(),
            name: name.into(),
        }
    }

    fn get_char(&self) -> LexResult<char> {
        let (row, col) = self.pos.as_tuple();
        self.lines
            .get(row)
            .and_then(|line| line.get(col))
            .cloned()
            .ok_or_else(|| LexError::EOF)
    }

    fn decrement_pos(&mut self) {
        // If we are beyond the final line, move to last char
        if self.pos.row >= self.lines.len() {
            self.pos.row = self.lines.len() - 1;
            self.pos.col = self.lines[self.pos.row].len() - 1;
            return;
        }

        // Decrement line if we reach beginning of a line
        if self.pos.col == 0 {
            self.pos.row -= 1;
            self.pos.col = self.lines[self.pos.row].len() - 1;
            return;
        }

        // Otherwise just decrement column
        self.pos.col -= 1;
    }

    fn advance_pos(&mut self) -> Option<()> {
        let line = self.lines.get(self.pos.row)?;
        if self.pos.col >= line.len() - 1 {
            self.pos.col = 0;
            self.pos.row += 1;
        } else {
            self.pos.col += 1;
        }
        Some(())
    }

    pub fn next_char(&mut self) -> LexResult<char> {
        let ch = self.get_char()?;
        self.advance_pos();
        Ok(ch)
    }

    fn try_run<T>(&mut self, func: impl Fn(&mut Lexer) -> LexResult<T>) -> LexResult<T> {
        let start_pos = self.pos.clone();

        let res = func(self);

        // Reset if there was an error
        if res.is_err() {
            self.pos = start_pos;
        }

        res
    }

    fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> SpanData<String> {
        let start = self.pos.clone();
        let mut value = String::new();

        while let Ok(ch) = self.next_char() {
            if !predicate(ch) {
                self.decrement_pos();
                break;
            }
            value.push(ch);
        }

        let stop = self.pos.clone();
        let span = Span {
            name: self.name.clone(),
            start,
            stop,
        };
        SpanData {
            span,
            value: value.into(),
        }
    }

    pub fn try_parse_symbol(&mut self) -> LexResult<SpanData<Token>> {
        self.try_run(|lexer| {
            let tree = get_symbol_tree();
            let symbols = get_symbol_chars();
            let symbol_text = lexer.read_while(|ch| symbols.contains(&ch));
            let symbol_token = tree
                .find(&symbol_text.value)
                .map(|token| token.clone())
                .ok_or_else(|| LexError::unknown_symbol(symbol_text.value.as_str()))?;
            Ok(SpanData {
                span: symbol_text.span,
                value: symbol_token,
            })
        })
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
