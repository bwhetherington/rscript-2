pub type Str = std::sync::Arc<str>;

#[derive(Debug, Clone)]
pub struct Point {
    pub row: usize,
    pub col: usize,
}

impl Point {
    pub fn increment_col(&mut self) {
        self.col += 1;
    }

    pub fn increment_row(&mut self) {
        self.col = 0;
        self.row += 1;
    }

    pub fn as_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }
}

impl From<(usize, usize)> for Point {
    fn from((row, col): (usize, usize)) -> Point {
        Point { row, col }
    }
}

#[derive(Clone, Debug)]
pub struct Span {
    pub name: Str,
    pub start: Point,
    pub stop: Point,
}

impl Span {
    pub fn join(&self, other: &Span) -> Span {
        Span {
            name: self.name.clone(),
            start: other.start.clone(),
            stop: self.stop.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SpanData<T> {
    pub span: Span,
    pub value: T,
}

pub enum TypeExpression {
    Identifier(Str),
}

pub enum Expression {
    Number(f64),
    String(Str),
    Identifier(Str),
}

pub enum BinaryOp {
    Plus,
    Minus,
    Times,
    Divide,
    Modulo,
    Equals,
    NotEquals,
    LT,
    LTE,
    GT,
    GTE,
}

pub struct Typed<T> {
    type_value: TypeExpression,
    value: T,
}

pub enum Statement {
    Declaration {
        public: bool,
        name: Str,
        value: SpanData<Expression>,
    },
    Function {
        public: bool,
        name: Str,
        args: Vec<Typed<Str>>,
        body: Vec<SpanData<Statement>>,
    },
    Expression(SpanData<Expression>),
}
