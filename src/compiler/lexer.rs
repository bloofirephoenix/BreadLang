#[derive(PartialEq, Debug)]
pub enum Instruction {
    NOP  = 0b0000,
    LW   = 0b0001,
    SW   = 0b0010,
    MW   = 0b0011,
    PUSH = 0b0100,
    POP  = 0b0101,
    LDA  = 0b0110,
    JMP  = 0b0111,
    JZ   = 0b1000,
    JO   = 0b1001,
    ADD  = 0b1010,
    SUB  = 0b1011,
    //     0b1100
    TEL  = 0b1101,
    OUT  = 0b1110,
    HLT  = 0b1111,
}

#[derive(PartialEq, Debug)]
pub enum TokenType {
    // literals
    Identifier(String),
    Number(String),

    // 1 line char tokens
    Comma,
    Colon,
    OpenParenthesis,
    CloseParenthesis,

    // keywords
    Macro, 
    Include,
    Def,
    Register(Register),

    Instruction(Instruction),

    Indent(String),
    EndOfFile,
    NewLine
}

#[derive(PartialEq, Debug)]
pub enum Register {
    A,
    B,
    H,
    L
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
}

impl Token {
    fn new(token_type: TokenType, line: i32) -> Token {
        Token {
            token_type,
            line
        }
    }
}

struct Tokenizer {
    start: usize,
    current: usize,
    line: i32,
    tokens: Vec<Token>,
    chars: Vec<char>
}

impl Tokenizer {
    pub fn new(text: String) -> Tokenizer {
        Tokenizer {
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            chars: text.chars().collect()
        }
    }

    pub fn char(&self) -> char {
        self.chars[self.current]
    }

    pub fn peek(&self) -> char {
        if self.is_at_end() {
            '\0'
        } else {
            self.chars[self.current]
        }
    }

    pub fn advance(&mut self) -> char {
        self.current += 1;
        self.chars[self.current - 1]
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.chars.len()
    }

    pub fn add_token(&mut self, token: TokenType) {
        self.tokens.push(Token::new(token, self.line))
    }

    pub fn get_string(&self) -> String {
        let mut string = String::from("");
        for i in self.start..self.current {
            string += &self.chars[i].to_string();
        }
        
        string
    }
}

pub fn scan_tokens(text: String) -> Vec<Token> {

    let mut tokenizer = Tokenizer::new(text);

    while !tokenizer.is_at_end() {
        tokenizer.start = tokenizer.current;
        scan_token(&mut tokenizer)
    }

    tokenizer.tokens.push(Token::new(TokenType::EndOfFile, tokenizer.line));

    tokenizer.tokens
}

fn scan_token(tokenizer: &mut Tokenizer) {
    match tokenizer.advance() {

        // 1 line chars
        ',' => tokenizer.add_token(TokenType::Comma),
        ':' => tokenizer.add_token(TokenType::Colon),
        '(' => tokenizer.add_token(TokenType::OpenParenthesis),
        ')' => tokenizer.add_token(TokenType::CloseParenthesis),

        // comments
        ';' => {
            while tokenizer.peek() != '\n' && !tokenizer.is_at_end() {
                tokenizer.advance();
            }
        }

        // indents
        ' ' | '\t' => 'indent: {
            if tokenizer.tokens.len() > 0 && tokenizer.tokens.last().unwrap().token_type != TokenType::NewLine {
                break 'indent;
            }
            
            while tokenizer.peek().is_whitespace() && tokenizer.peek() != '\n' && tokenizer.peek() != '\r' {
                tokenizer.advance();
            }
            tokenizer.add_token(TokenType::Indent(tokenizer.get_string()))
        }

        '\r' => (), // ignore
        '\n' => {
            tokenizer.line += 1;
            tokenizer.add_token(TokenType::NewLine);
        }

        _ => {
             if tokenizer.char().is_digit(10) {
                number(tokenizer)
            } else if is_alphabetic(tokenizer.char()) {
                identifier(tokenizer);
            } else {
                panic!("Unexpected Character");
            }
        }
    }
}

fn number(tokenizer: &mut Tokenizer) {
    if tokenizer.char() == '0' {
        match tokenizer.peek() {
            // hex number
            'x' | 'X' => {
                tokenizer.advance();
                while tokenizer.peek().is_digit(16) {
                    tokenizer.advance();
                }
            }

            // binary number
            'b' | 'B' => {
                tokenizer.advance();
                while tokenizer.peek().is_digit(2) || tokenizer.peek() == '_' {
                    tokenizer.advance();
                }
            }
            _ => panic!("Expected binary or hex number")
        }
    } else {
        while tokenizer.peek().is_digit(10) || tokenizer.peek() == '_' {
            tokenizer.advance();
        }
    }

    tokenizer.add_token(TokenType::Number(tokenizer.get_string()));
}

fn identifier(tokenizer: &mut Tokenizer) {
    while is_alphanumeric(tokenizer.peek()) {
        tokenizer.advance();
    }

    match tokenizer.get_string().as_str() {
        // instructions
        "NOP" => tokenizer.add_token(TokenType::Instruction(Instruction::NOP)),
        "LW" => tokenizer.add_token(TokenType::Instruction(Instruction::LW)),
        "SW" => tokenizer.add_token(TokenType::Instruction(Instruction::SW)),
        "MW" => tokenizer.add_token(TokenType::Instruction(Instruction::MW)),
        "PUSH" => tokenizer.add_token(TokenType::Instruction(Instruction::PUSH)),
        "POP" => tokenizer.add_token(TokenType::Instruction(Instruction::POP)),
        "LDA" => tokenizer.add_token(TokenType::Instruction(Instruction::LDA)),
        "JMP" => tokenizer.add_token(TokenType::Instruction(Instruction::JMP)),
        "JZ" => tokenizer.add_token(TokenType::Instruction(Instruction::JZ)),
        "JO" => tokenizer.add_token(TokenType::Instruction(Instruction::JO)),
        "ADD" => tokenizer.add_token(TokenType::Instruction(Instruction::ADD)),
        "SUB" => tokenizer.add_token(TokenType::Instruction(Instruction::SUB)),
        "TEL" => tokenizer.add_token(TokenType::Instruction(Instruction::TEL)),
        "OUT" => tokenizer.add_token(TokenType::Instruction(Instruction::OUT)),
        "HLT" => tokenizer.add_token(TokenType::Instruction(Instruction::HLT)),
        
        // keywords
        "@macro" => tokenizer.add_token(TokenType::Macro),
        "@include" => tokenizer.add_token(TokenType::Include),
        "DEF" => tokenizer.add_token(TokenType::Def),

        "A" => tokenizer.add_token(TokenType::Register(Register::A)),
        "B" => tokenizer.add_token(TokenType::Register(Register::B)),
        "H" => tokenizer.add_token(TokenType::Register(Register::H)),
        "L" => tokenizer.add_token(TokenType::Register(Register::L)),

        _ => tokenizer.add_token(TokenType::Identifier(tokenizer.get_string()))
    }
}

fn is_alphanumeric(char: char) -> bool {
    is_alphabetic(char) || char.is_digit(10)
}

fn is_alphabetic(char: char) -> bool {
    (char >= 'a' && char <= 'z') || (char >= 'A' && char <= 'Z') || char == '_' || char == '@'
}