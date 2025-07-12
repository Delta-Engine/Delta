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

    // Operators
    Plus,
    Minus,
    Multiply,
    Divide,
    
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

    fn advance(&mut self) {
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
    /*
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
    */

    fn try_match_keyword(&mut self, keyword: &str) -> bool {
        let saved_position = self.position;
        let saved_char = self.current_char;
        let saved_line = self.line;
        let saved_column = self.column;

        let words: Vec<&str> = keyword.split(' ').collect();

        for (i, word) in words.iter().enumerate() {
            if (i > 0) {
                self.skip_whitespace();
            }
            
            let read_word = self.read_identifier();

            if read_word != *word {
                self.position = saved_position;
                self.current_char = saved_char;
                self.line = saved_line;
                self.column = saved_column;

                return false;
            }
        }

        true
    }

    fn keyword_or_identifier(&mut self) -> Token {
        // Try to match multi-word keywords first (longest first)
        let keywords = [
            ("is greater than or equal", Token::IsGreaterThanOrEqual),
            ("is less than or equal", Token::IsLessThanOrEqual),
            ("is greater than", Token::IsGreaterThan),
            ("is less than", Token::IsLessThan),
            ("is not equal", Token::IsNotEqual),
            ("is equal", Token::IsEqual),
        ];
        
        for (keyword, token) in keywords.iter() {
            if self.try_match_keyword(keyword) {
                return token.clone();
            }
        }
        
        // Try single-word keywords
        let word = self.read_identifier();
        match word.as_str() {
            "let" => Token::Let,
            "be" => Token::Be,
            "when" => Token::When,
            "then" => Token::Then,
            "otherwise" => Token::Otherwise,
            "show" => Token::Show,
            "define" => Token::Define,
            "with" => Token::With,
            "end" => Token::End,
            _ => Token::Identifier(word),
        }
    }

    // The below two functions are AI Generated.

    fn handle_newline_and_indentation(&mut self) -> Vec<Token> {
        let mut tokens = vec![Token::Newline];
        self.advance(); // Skip the newline
        
        // Count indentation
        let mut indent_level = 0;
        while let Some(ch) = self.current_char {
            if ch == ' ' {
                indent_level += 1;
                self.advance();
            } else if ch == '\t' {
                indent_level += 4; // Treat tab as 4 spaces
                self.advance();
            } else {
                break;
            }
        }
        
        let current_indent = *self.indent_stack.last().unwrap();
        
        if indent_level > current_indent {
            self.indent_stack.push(indent_level);
            tokens.push(Token::Indent);
        } else if indent_level < current_indent {
            while let Some(&last_indent) = self.indent_stack.last() {
                if last_indent <= indent_level {
                    break;
                }
                self.indent_stack.pop();
                tokens.push(Token::Dedent);
            }
        }
        
        tokens
    }
    
    pub fn tokenize(&mut self) -> Result<Vec<Token>, String> {
        let mut tokens = Vec::new();
        
        while let Some(ch) = self.current_char {
            match ch {
                ' ' | '\t' => self.skip_whitespace(),
                '\n' => {
                    let newline_tokens = self.handle_newline_and_indentation();
                    tokens.extend(newline_tokens);
                }
                '"' => {
                    let string = self.read_string()?;
                    tokens.push(Token::String(string));
                }
                '0'..='9' => {
                    let number = self.read_number();
                    tokens.push(Token::Number(number));
                }
                'a'..='z' | 'A'..='Z' | '_' => {
                    let token = self.keyword_or_identifier();
                    tokens.push(token);
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.advance();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.advance();
                }
                '*' => {
                    tokens.push(Token::Multiply);
                    self.advance();
                }
                '/' => {
                    tokens.push(Token::Divide);
                    self.advance();
                }
                _ => {
                    return Err(format!("Unexpected character: '{}' at line {}, column {}", 
                                     ch, self.line, self.column));
                }
            }
        }
        
        // Add final dedents for any remaining indentation
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            tokens.push(Token::Dedent);
        }
        
        tokens.push(Token::Eof);
        Ok(tokens)
    }
}