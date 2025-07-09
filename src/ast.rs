#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statements>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Let(LetStatement),
    Show(ShowStatement),
    When(WhenStatement),
    FunctionDef(FunctionDef),
    Expression(Expression),
}