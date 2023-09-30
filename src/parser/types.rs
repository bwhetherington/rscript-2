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

pub enum BinaryOperator {
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

pub enum UnaryOperator {
    Negative,
    Not,
}

pub struct Unary {
    operator: UnaryOperator,
    target: Box<Expression>,
}

pub struct Binary {
    operator: BinaryOperator,
    lhs: Box<Expression>,
    rhs: Box<Expression>,
}

pub struct Block {
    body: Vec<SpanData<Statement>>,
    value: Option<Box<Expression>>,
}

pub struct If {
    condition: Box<Expression>,
    then: Option<Block>,
    otherwise: Option<Block>,
}

pub enum Expression {
    Number(f64),
    String(Str),
    Identifier(Str),
    Unary(Unary),
    Binary(Binary),
}

pub struct Typed<T> {
    type_expr: Option<TypeExpression>,
    value: T,
}

pub enum Visibility {
    Public,
    Private,
}

impl Visibility {
    pub fn is_public(&self) -> bool {
        match self {
            Visibility::Public => true,
            _ => false,
        }
    }
}

pub struct Declaration {
    visibility: Visibility,
    name: Typed<Str>,
    value: SpanData<Expression>,
}

pub struct Function {
    visibility: Visibility,
    name: Str,
    args: Vec<Typed<Str>>,
    body: Block,
}

pub enum Statement {
    Declaration(Declaration),
    Function(Function),
    Expression(SpanData<Expression>),
}

pub enum ParseError {
    EOF,
    Custom(Str),
}

impl ParseError {
    pub fn custom(msg: impl Into<Str>) -> ParseError {
        ParseError::Custom(msg.into())
    }
}

pub type ParseResult<T> = Result<T, ParseError>;
