#[derive(Debug)]
pub enum Token {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,

    Dot,
    Minus,
    Plus,
    Star,
    Slash,

    Comma,
    Semicolon,

    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    And,
    Or,

    Class,
    This,
    Super,

    For,
    While,

    If,
    Else,

    Fun,
    Return,
    Print,

    Var,
    String,
    Number,
    Nil,
    True,
    False,

    Identifier,

    EOF,
}

#[derive(Debug)]
pub struct TokenData<'a> {
    token: &'a Token,
    lexeme: &'a str,
    offset: usize,
    line: usize,
    column: usize,
}

impl<'a> TokenData<'a> {
    pub fn new(
        token: &'a Token,
        lexeme: &'a str,
        offset: usize,
        line: usize,
        column: usize,
    ) -> Self {
        Self {
            token,
            lexeme,
            offset,
            line,
            column,
        }
    }
}

pub struct Scanner<'a> {
    code: &'a str,
    start: usize,
    next_offset: usize,
    column: usize,
    line: usize,
    token_data_list: Vec<TokenData<'a>>,
}

impl<'a> Scanner<'a> {
    pub fn new(code: &'a str) -> Self {
        Self {
            code,
            start: 0,
            next_offset: 0,
            column: 0,
            line: 1,
            token_data_list: Vec::new(),
        }
    }

    pub fn scan(&mut self) -> &Vec<TokenData<'a>> {
        loop {
            self.column += self.next_offset - self.start;
            self.start = self.next_offset;
            let c = self.advance();

            if !self.scan_token(c) {
                break;
            }
        }

        &self.token_data_list
    }

    fn scan_token(&mut self, c: Option<&'a str>) -> bool {
        match c {
            Some(c) => {
                let c = c.chars().next().unwrap();

                match c {
                    '(' => self.add_token(&Token::LeftParen),
                    ')' => self.add_token(&Token::RightParen),
                    '{' => self.add_token(&Token::LeftBrace),
                    '}' => self.add_token(&Token::RightBrace),
                    '.' => self.add_token(&Token::Dot),
                    ',' => self.add_token(&Token::Comma),
                    ';' => self.add_token(&Token::Semicolon),
                    '-' => self.add_token(&Token::Minus),
                    '+' => self.add_token(&Token::Plus),
                    '*' => self.add_token(&Token::Star),

                    '!' => self.add_token_if_next("=", &Token::BangEqual, &Token::Bang),
                    '=' => self.add_token_if_next("=", &Token::EqualEqual, &Token::Equal),
                    '>' => self.add_token_if_next("=", &Token::GreaterEqual, &Token::Greater),
                    '<' => self.add_token_if_next("=", &Token::LessEqual, &Token::Less),

                    '/' => self.add_token_division_or_comment(),

                    '\n' => self.mark_new_line(),

                    '"' => {
                        if !self.add_token_string() {
                            // TODO Report error
                        }
                    }

                    x if x.is_ascii_digit() => self.add_token_number(),
                    x if x.is_ascii_alphabetic() => self.add_token_identifier_or_keyword(),
                    x if x.is_whitespace() => {} // Skip whitespace
                    x => {
                        // TODO Report error
                    }
                }
            }
            None => {
                self.add_token(&Token::EOF);
                return false;
            }
        }

        true
    }

    fn add_token_division_or_comment(&mut self) {
        if self.is_next("/") {
            self.ignore_until_end();
        } else {
            self.add_token(&Token::Slash);
        }
    }

    fn advance(&mut self) -> Option<&'a str> {
        if self.next_offset >= self.code.len() {
            return None;
        }

        let start = self.next_offset;
        let end = start + 1;

        self.next_offset += 1;

        Some(&self.code[start..end])
    }

    fn is_next(&self, str: &str) -> bool {
        self.is_next_cond(|c| c == str.chars().next().unwrap())
    }

    fn is_next_cond(&self, condition_fn: impl FnOnce(char) -> bool) -> bool {
        if self.next_offset >= self.code.len() {
            return false;
        }

        let next_char = self.code[self.next_offset..].chars().next().unwrap();
        condition_fn(next_char)
    }

    fn get_lexeme(&self) -> &'a str {
        &self.code[self.start..self.next_offset]
    }

    fn add_token(&mut self, token: &'a Token) {
        self.token_data_list.push(TokenData {
            token,
            lexeme: self.get_lexeme(),
            offset: self.start,
            column: self.column,
            line: self.line,
        });
    }

    fn add_token_if_next(&mut self, next_str: &str, true_token: &'a Token, false_token: &'a Token) {
        if self.is_next(next_str) {
            self.advance();
            self.add_token(true_token);
        } else {
            self.add_token(false_token);
        }
    }

    fn add_token_string(&mut self) -> bool {
        loop {
            break match self.advance() {
                Some("\\") => {
                    self.advance();
                    continue;
                }
                Some("\"") => {
                    self.add_token(&Token::String);
                    true
                }
                Some(_) => continue,
                None => false,
            };
        }
    }

    fn add_token_identifier_or_keyword(&mut self) {
        while self.is_next_cond(|c| c.is_alphanumeric() || c == '_') {
            self.advance();
        }

        let lexeme = self.get_lexeme();
        let token = match lexeme {
            "and" => &Token::And,
            "or" => &Token::Or,
            "class" => &Token::Class,
            "this" => &Token::This,
            "super" => &Token::Super,
            "for" => &Token::For,
            "while" => &Token::While,
            "if" => &Token::If,
            "else" => &Token::Else,
            "fun" => &Token::Fun,
            "return" => &Token::Return,
            "print" => &Token::Print,
            "var" => &Token::Var,
            "nil" => &Token::Nil,
            "true" => &Token::True,
            "false" => &Token::False,
            _ => &Token::Identifier,
        };

        self.add_token(token);
    }

    fn add_token_number(&mut self) {
        while self.is_next_cond(|c| c.is_ascii_digit()) {
            self.advance();
        }

        if self.is_next(".") {
            self.advance();

            while self.is_next_cond(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }

        self.add_token(&Token::Number);
    }

    fn ignore_until_end(&mut self) {
        while !self.is_next("\n") && !self.is_eof() {
            self.advance();
        }
    }

    fn is_eof(&self) -> bool {
        self.next_offset >= self.code.len()
    }

    fn mark_new_line(&mut self) {
        self.start += 1; // To ignore new line
        self.column = 0;
        self.line += 1;
    }
}
