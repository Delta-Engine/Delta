#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Show(ShowStatement),
    When(WhenStatement),
    FunctionDef(FunctionDef),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LetStatement {
    pub identifier: String,
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShowStatement {
    pub value: Expression,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WhenStatement {
    pub condition: Expression,
    pub then_block: Vec<Statement>,
    pub otherwise_block: Option<Vec<Statement>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDef {
    pub name: String,
    pub parameters: Vec<String>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    String(String),
    Identifier(String),
    BinaryOp(BinaryOperation),
    FunctionCall(FunctionCall),
}

#[derive(Debug, Clone, PartialEq)]
pub struct BinaryOperation {
    pub left: Box<Expression>,
    pub operator: BinaryOperator,
    pub right: Box<Expression>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Equal,
    NotEqual,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}