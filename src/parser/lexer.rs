use std::{collections::HashSet, sync::OnceLock};

use regex::Regex;

use crate::parser::{Point, PrefixTree, Span, SpanData, Str};

#[derive(Clone, Debug)]
pub enum Token {
    Number(f64),
    Boolean(bool),
    String(Str),

    // Word tokens
    Identifier(Str),
    Public,
    Function,
    Let,
    If,
    Else,
    Loop,
    While,
    For,
    In,
    Return,
    Break,
    Continue,

    // Symbol tokens
    ExclusiveRange,
    InclusiveRange,
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
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Times,
    TimesEquals,
    Divide,
    DivideEquals,
    Modulo,
    ModuloEquals,
    GreaterThan,
    GreaterThanEquals,
    LessThan,
    LessThanEquals,
    NotEquals,
    Not,
    SingleArrow,
    DoubleArrow,
}

static WORD_TREE: OnceLock<PrefixTree<Token>> = OnceLock::new();

fn get_word_tree() -> &'static PrefixTree<Token> {
    WORD_TREE.get_or_init(|| {
        PrefixTree::from_iter([
            ("pub", Token::Public),
            ("fn", Token::Function),
            ("let", Token::Let),
            ("if", Token::If),
            ("else", Token::Else),
            ("loop", Token::Loop),
            ("while", Token::While),
            ("for", Token::For),
            ("in", Token::In),
            ("break", Token::Break),
            ("continue", Token::Continue),
            ("return", Token::Return),
        ])
    })
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
            ("+", Token::Plus),
            ("+=", Token::PlusEquals),
            ("-", Token::Minus),
            ("-=", Token::MinusEquals),
            ("*", Token::Times),
            ("*=", Token::TimesEquals),
            ("/", Token::Divide),
            ("/=", Token::DivideEquals),
            ("%", Token::Modulo),
            ("%=", Token::ModuloEquals),
            (">", Token::GreaterThan),
            (">=", Token::GreaterThanEquals),
            ("<", Token::LessThan),
            ("<=", Token::LessThanEquals),
            ("==", Token::DoubleEquals),
            ("!=", Token::NotEquals),
            ("!", Token::Not),
            ("..", Token::ExclusiveRange),
            ("..=", Token::InclusiveRange),
            ("->", Token::SingleArrow),
            ("=>", Token::DoubleArrow),
        ])
    })
}

static SYMBOL_CHARS: OnceLock<HashSet<char>> = OnceLock::new();

fn get_symbol_chars() -> &'static HashSet<char> {
    SYMBOL_CHARS.get_or_init(|| get_symbol_tree().get_all_chars())
}

fn is_atom_first_char(ch: char) -> bool {
    ch.is_ascii_alphabetic() || ch == '_'
}

fn is_atom_char(ch: char) -> bool {
    ch.is_ascii_digit() || is_atom_first_char(ch)
}

#[derive(Debug)]
pub enum LexError {
    Eof,
    ExpectedNumber,
    ExpectedAtom,
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

static IDENTIFIER_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_identifier_regex() -> &'static Regex {
    IDENTIFIER_REGEX.get_or_init(|| {
        Regex::new(r"[a-zA-Z_][a-zA-Z0-9_]*").expect("failed to compile identifier regex")
    })
}

static NUMBER_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_number_regex() -> &'static Regex {
    NUMBER_REGEX
        .get_or_init(|| Regex::new(r"[0-9]*(\.[0-9]+)?").expect("failed to compile number regex"))
}

fn is_whitespace(ch: char) -> bool {
    ch.is_whitespace()
}

fn not<T>(f: impl Fn(T) -> bool) -> impl Fn(T) -> bool {
    move |x| !f(x)
}

fn is_numeric(ch: char) -> bool {
    ch.is_ascii_digit()
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
            .ok_or_else(|| LexError::Eof)
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

    fn next_char(&mut self) -> LexResult<char> {
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

    fn read_while_to(&mut self, predicate: impl Fn(char) -> bool, buf: &mut SpanData<String>) {
        while let Ok(ch) = self.next_char() {
            if !predicate(ch) {
                self.decrement_pos();
                break;
            }
            buf.value.push(ch);
        }

        let stop = self.pos.clone();
        buf.span.stop = stop;
    }

    fn read_while(&mut self, predicate: impl Fn(char) -> bool) -> SpanData<String> {
        let mut res = SpanData {
            span: Span {
                name: self.name.clone(),
                start: self.pos.clone(),
                stop: self.pos.clone(),
            },
            value: String::new(),
        };
        self.read_while_to(predicate, &mut res);
        res
    }

    fn try_parse_string(&mut self) -> LexResult<SpanData<Token>> {
        self.try_run(|lexer| {
            let start = lexer.pos.clone();
            lexer.try_parse_char(|ch| ch == '"')?;
            let mut buf = String::new();

            let mut prev_ch = '"';
            while let Ok(ch) = lexer.next_char() {
                match (prev_ch, ch) {
                    ('\\', ch) => {
                        buf.push(ch);
                    }
                    (_, '"') => {
                        break;
                    }
                    (_, ch) => {
                        buf.push(ch);
                    }
                }
                prev_ch = ch;
            }

            let mut span = lexer.empty_span();
            span.start = start;
            Ok(SpanData {
                span,
                value: Token::String(buf.into()),
            })
        })
    }

    fn try_parse_symbol(&mut self) -> LexResult<SpanData<Token>> {
        self.try_run(|lexer| {
            let tree = get_symbol_tree();
            let symbols = get_symbol_chars();
            let symbol_text = lexer.read_while(|ch| symbols.contains(&ch));
            let symbol_token = tree
                .find(&symbol_text.value)
                .cloned()
                .ok_or_else(|| LexError::unknown_symbol(symbol_text.value.as_str()))?;
            Ok(SpanData {
                span: symbol_text.span,
                value: symbol_token,
            })
        })
    }

    fn empty_span(&self) -> Span {
        Span {
            name: self.name.clone(),
            start: self.pos.clone(),
            stop: self.pos.clone(),
        }
    }

    fn skip_whitespace(&mut self) {
        while let Ok(ch) = self.next_char() {
            if !is_whitespace(ch) {
                self.decrement_pos();
                break;
            }
        }
    }

    fn try_parse_atom(&mut self) -> LexResult<SpanData<Token>> {
        self.try_run(|lexer| {
            // Create empty SpanData to read into
            let mut res = SpanData {
                span: lexer.empty_span(),
                value: String::new(),
            };

            // Check first character
            let first_char = lexer
                .try_parse_char(is_atom_first_char)
                .map_err(|_| LexError::ExpectedAtom)?;
            res.value.push(first_char);
            res.span.stop = lexer.pos.clone();

            // Read all additional characters
            lexer.read_while_to(is_atom_char, &mut res);

            // Check if it's a word token
            let token = match get_word_tree().find(&res.value) {
                Some(token) => token.clone(),
                _ => Token::Identifier(res.value.into()),
            };

            Ok(SpanData {
                span: res.span,
                value: token,
            })
        })
    }

    fn try_parse_char(&mut self, predicate: impl Fn(char) -> bool) -> LexResult<char> {
        let ch = self.next_char()?;
        if predicate(ch) {
            Ok(ch)
        } else {
            self.decrement_pos();
            Err(LexError::custom("character failed predicate"))
        }
    }

    fn try_parse_number(&mut self) -> LexResult<SpanData<f64>> {
        self.try_run(|lexer| {
            let mut number = lexer.read_while(is_numeric);
            match lexer.next_char() {
                Ok('.') => {
                    let post_decimal = lexer.read_while(is_numeric);

                    // Check if we find any numbers after the .
                    if !post_decimal.value.is_empty() {
                        // If we didn't, then the . was not part of a decimal number
                        lexer.decrement_pos();
                        Ok(())
                    } else {
                        number.value.push('.');
                        number.value.push_str(&post_decimal.value);
                        number.span.stop = post_decimal.span.stop;
                        Ok(())
                    }
                }
                Ok(_) => {
                    lexer.decrement_pos();
                    Ok(())
                }
                Err(_) if !number.value.is_empty() => Ok(()),
                Err(why) => Err(why),
            }?;
            let number_value: f64 = number.value.parse().map_err(|_| LexError::ExpectedNumber)?;
            Ok(SpanData {
                span: number.span,
                value: number_value,
            })
        })
    }

    fn try_parse_number_token(&mut self) -> LexResult<SpanData<Token>> {
        self.try_run(|lexer| {
            let num = lexer.try_parse_number()?;
            Ok(SpanData {
                span: num.span,
                value: Token::Number(num.value),
            })
        })
    }

    fn is_done(&self) -> bool {
        self.get_char().is_err()
    }

    fn next_token(&mut self) -> LexResult<Option<SpanData<Token>>> {
        self.skip_whitespace();
        if !self.is_done() {
            let token = self
                .try_parse_atom()
                .or_else(|_| self.try_parse_string())
                .or_else(|_| self.try_parse_number_token())
                .or_else(|_| self.try_parse_symbol())?;
            Ok(Some(token))
        } else {
            Ok(None)
        }
    }

    pub fn try_parse_tokens(&mut self) -> LexResult<Vec<SpanData<Token>>> {
        let mut out = Vec::new();

        while let Some(token) = self.next_token()? {
            out.push(token);
        }

        Ok(out)
    }
}
