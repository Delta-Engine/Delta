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

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut lexer = Lexer {
            input,
            position: 0,
            current_char: None,
            line: 1,
            column: 0,
            indent_stack: vec![0],
        }; 
        lexer.current_char = lexer.input.chars().next();
        lexer
    }

    fn advance(mut self) {
        if self.current_char == Some('\n') {
            self.line = self.line + 1;
            self.column = 0;
        } else {
            self.column = self.column + 1;
        }

        self.position = self.position + 1;
        self.current_char = self.input.chars().nth(self.position);
    }   
}