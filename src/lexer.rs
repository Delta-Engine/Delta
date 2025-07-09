#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,
    Be,
    When,
    Then,
    Otherwise,
    Show,
    Define,
    With,
    End,
    
    // Comparators
    IsGreaterThan,
    IsLessThan,
    IsGreaterThanOrEqual,
    IsLessThanOrEqual,
    IsEqual,
    IsNotEqual,
    
    // Literals
    Number(f64),
    String(String),
    Identifier(String),
    
    // Whitespace and structure
    Newline,
    Indent,
    Dedent,
    
    // End of file
    Eof,
}


pub struct Lexer<'a> {
    input: &'a str,
    position: usize,
    current_char: Option<char>,
    line: usize,
    column: usize,
    indent_stack: Vec<usize>,
}