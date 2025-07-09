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

    fn peek(&self) -> Option<char> {
        self.input.chars().nth(self.position + 1)
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.current_char {
            if ch.is_whitespace() && ch != '\n' { // If Character is Whitespace,
                self.advance(); // Skip it.
            } else {
                break;
            }
        }
    }

    fn read_number(&mut self) -> f64 {
        let mut number = String::new();
        
        while let Some(ch) = self.current_char {
            if ch.is_ascii_digit() || ch == '.' { //Check for ASCII Digit OR Decimal
                number.push(ch);
                self.advance(); 
            } else {
                break;
            }
        }
        
        number.parse().unwrap_or(0.0)
    }

    fn read_string(&mut self) -> Result<String, String> {
        let mut string = String::new();
        self.advance(); // Go over the Starting Quote
        
        while let Some(ch) = self.current_char {
            if ch == '"' {
                self.advance(); // Go Over the Closing Quote
                return Ok(string);
            }
            if ch == '\\' {
                self.advance();
                match self.current_char {
                    Some('n') => string.push('\n'),
                    Some('t') => string.push('\t'),
                    Some('r') => string.push('\r'),
                    Some('\\') => string.push('\\'),
                    Some('"') => string.push('"'),
                    _ => return Err("Invalid escape sequence".to_string()),
                }
            } else {
                string.push(ch);
            }
            self.advance();
        }
        
        Err("Unterminated string".to_string())
    }

    fn read_identifier(&mut self) -> String {
        let mut identifier = String::new();

        while let Some(ch) = self.current_char {
            if ch.is_alphanumeric() || ch == '_' {
                identifier.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        identifier
    }

    fn read_multi_word_token(&mut self) -> String {
        let mut words = Vec::new();

        loop {
            let word = self.read_identifier();

            if word.is_empty() {
                break;
            }
            words.push(word);
            self.skip_whitespace();

            if let Some(ch) = self.current_char {
                if !ch.is_alphanumeric() {
                    break;
                }
            } else {
                break;
            }
        }

        words.join(" ")
    }

    fn keyword_or_identifier(&mut self) -> Token {
        let text = self.read_multi_word_token();

        match text.as_str() {
            "let" => Token::Let,
            "be" => Token::Be,
            "when" => Token::When,
            "then" => Token::Then,
            "otherwise" => Token::Otherwise,
            "show" => Token::Show,
            "define" => Token::Define,
            "with" => Token::With,
            "end" => Token::End,
            "is greater than" => Token::IsGreaterThan,
            "is less than" => Token::IsLessThan,
            "is greater than or equal" => Token::IsGreaterThanOrEqual,
            "is less than or equal" => Token::IsLessThanOrEqual,
            "is equal" => Token::IsEqual,
            "is not equal" => Token::IsNotEqual,
            _ => Token::Identifier(text),
        }
    }
}