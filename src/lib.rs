use std::collections::{HashMap, VecDeque};
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Push(u8),
    Pop(),
    Plus(),
    Minus(),
    Mult(),
    NumOut(),
    CharOut(),
    Write(),
    Read(),
    Mem(),
    InitMem(),
    If(Option<usize>),
    End(Option<usize>),
    Else(Option<usize>),
    While(Option<usize>),
    LessThan(),
    GreaterThan(),
    Equals(),
    Swap(),
    Dup(),
    GetStackHeight(),
    Not(),
    FunctionDeclaration(String),
    FunctionCall(String),
    StringLiteral(String),
    Import(String),
}

#[derive(Debug)]
pub struct Stack {
    pub data: [u8; 30000 - 256],
    pub top: usize,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: [0u8; 30000 - 256],
            top: 0,
        }
    }

    pub fn push(&mut self, byte: u8) {
        self.top += 1;
        self.data[self.top] = byte;
    }

    pub fn pop(&mut self) -> u8 {
        let byte = self.data[self.top];
        self.top -= 1;
        byte
    }
}

pub fn parse_tokens(tokens: Vec<Token>) -> Result<Vec<Function>, Box<dyn Error>> {
    let mut parsed_tokens: Vec<Function> = vec![];

    let mut token_iter = tokens.iter();

    while let Some(token) = token_iter.next() {
        if let Ok(number) = token.value.parse::<u8>() {
            parsed_tokens.push(Function::Push(number));
        } else if token.value == "pop" {
            parsed_tokens.push(Function::Pop());
        } else if token.value == "+" {
            parsed_tokens.push(Function::Plus());
        } else if token.value == "-" {
            parsed_tokens.push(Function::Minus());
        } else if token.value == "*" {
            parsed_tokens.push(Function::Mult());
        } else if token.value == "chout" {
            parsed_tokens.push(Function::CharOut());
        } else if token.value == "numout" {
            parsed_tokens.push(Function::NumOut());
        } else if token.value == "write" {
            parsed_tokens.push(Function::Write());
        } else if token.value == "read" {
            parsed_tokens.push(Function::Read());
        } else if token.value == "mem" {
            parsed_tokens.push(Function::Mem());
        } else if token.value == "initmem" {
            parsed_tokens.push(Function::InitMem());
        } else if token.value == "if" {
            parsed_tokens.push(Function::If(None));
        } else if token.value == "end" {
            parsed_tokens.push(Function::End(None));
        } else if token.value == "else" {
            parsed_tokens.push(Function::Else(None));
        } else if token.value == "while" {
            parsed_tokens.push(Function::While(None));
        } else if token.value == "<" {
            parsed_tokens.push(Function::LessThan());
        } else if token.value == ">" {
            parsed_tokens.push(Function::GreaterThan());
        } else if token.value == "=" {
            parsed_tokens.push(Function::Equals());
        } else if token.value == "swap" {
            parsed_tokens.push(Function::Swap());
        } else if token.value == "dup" {
            parsed_tokens.push(Function::Dup());
        } else if token.value == "?" {
            parsed_tokens.push(Function::GetStackHeight());
        } else if token.value == "not" {
            parsed_tokens.push(Function::Not());
        } else if token.value == "fn" {
            if let Some(token) = token_iter.next() {
                parsed_tokens.push(Function::FunctionDeclaration(token.value.to_string()));
            }
        } else if token.value == "import" {
            if let Some(token) = token_iter.next() {
                parsed_tokens.push(Function::Import(token.value.to_string()));
            }
        } else if token.value.contains("\"") {
            let string_value = &token.value[1..(token.value.len() - 1)];
            parsed_tokens.push(Function::StringLiteral(string_value.to_string()));
        } else {
            parsed_tokens.push(Function::FunctionCall(token.value.to_string()));

            /*
            eprintln!("{}:{}:{} could not parse token: '{}'",
                token.filepath,
                token.row,
                token.col,
                token.value
            ); // Need to improve better error reporting
            return Err("Syntax error")?;
            */
        }
    }
    Ok(parsed_tokens)
}

pub struct Program {
    pub imports: VecDeque<String>,
    pub functions: HashMap<String, Vec<Function>>,
}

impl Program {
    pub fn new(imports: VecDeque<String>, functions: HashMap<String, Vec<Function>>) -> Program {
        Program { imports, functions }
    }

    pub fn consume(&mut self, mut program: Program) {
        // add all imports to self
        while let Some(q_item) = &program.imports.pop_front() {
            self.imports.push_back(q_item.to_string());
        }

        // Add all functions of program to self (non overriding)
        // TODO: Make this non overriding
        for (k, v) in program.functions {
            self.functions.insert(k, v);
        }
    }
}

pub fn parse_program_structure(parsed_tokens: Vec<Function>) -> Result<Program, Box<dyn Error>> {
    let mut functions: HashMap<String, Vec<Function>> = HashMap::new();
    let mut q: VecDeque<String> = VecDeque::new();

    let mut block_tokens: Vec<(usize, u8)> = Vec::new();
    let mut function_tokens: Vec<Function> = Vec::new();

    let mut function_name: String = String::new();

    const IF: u8 = 0;
    const ELSE: u8 = 1;
    const WHILE: u8 = 2;
    const FUNCDEF: u8 = 3;

    for i in 0..parsed_tokens.len() {
        match &parsed_tokens[i] {
            Function::If(_reference) => {
                block_tokens.push((i, IF));
                function_tokens.push(parsed_tokens[i].clone());
            }
            Function::Else(_reference) => {
                block_tokens.push((i, ELSE));
                function_tokens.push(parsed_tokens[i].clone());
            }
            Function::While(_reference) => {
                block_tokens.push((i, WHILE));
                function_tokens.push(parsed_tokens[i].clone());
            }
            Function::FunctionDeclaration(fname) => {
                function_name = fname.to_string();
                block_tokens.push((i, FUNCDEF));
            }
            Function::End(_reference) => {
                let (_index, block_word_type) = block_tokens[block_tokens.len() - 1];
                if block_word_type == FUNCDEF {
                    functions.insert(function_name, function_tokens);
                    function_tokens = Vec::new();
                    function_name = String::new();
                } else if block_word_type == ELSE {
                    let _tk = block_tokens.pop();
                    let _tk = block_tokens.pop();
                    function_tokens.push(parsed_tokens[i].clone());
                } else {
                    let _tk = block_tokens.pop();
                    function_tokens.push(parsed_tokens[i].clone());
                }
            }
            Function::Import(filename) => {
                q.push_back(filename.clone());
            }
            _ => {
                function_tokens.push(parsed_tokens[i].clone());
            }
        }
    }
    Ok(Program::new(q, functions))
}

pub fn create_references_for_blocks(parsed_tokens: &mut Vec<Function>) {
    let mut block_tokens: Vec<(usize, u8)> = vec![];

    const IF: u8 = 0;
    const ELSE: u8 = 1;
    const WHILE: u8 = 2;

    for i in 0..parsed_tokens.len() {
        match parsed_tokens[i] {
            Function::If(_reference) => {
                block_tokens.push((i, IF));
            }
            Function::Else(_reference) => {
                let (index, block_word_type) = block_tokens[block_tokens.len() - 1];
                if block_word_type == IF {
                    parsed_tokens[index] = Function::If(Some(i + 1));
                    block_tokens.push((i, ELSE));
                }
            }
            Function::End(_reference) => {
                let (index, block_word_type) = block_tokens[block_tokens.len() - 1];
                if block_word_type == IF {
                    parsed_tokens[index] = Function::If(Some(i));
                    parsed_tokens[i] = Function::End(Some(i + 1));
                    let _tk = block_tokens.pop();
                } else if block_word_type == ELSE {
                    parsed_tokens[index] = Function::Else(Some(i));
                    parsed_tokens[i] = Function::End(Some(i + 1));
                    let _tk = block_tokens.pop();
                    let _tk = block_tokens.pop();
                } else if block_word_type == WHILE {
                    parsed_tokens[index] = Function::While(Some(i + 1));
                    parsed_tokens[i] = Function::End(Some(index));
                    let _tk = block_tokens.pop();
                }
            }
            Function::While(_reference) => {
                block_tokens.push((i, WHILE));
            }
            _ => {}
        }
    }
}

#[derive(Debug)]
pub struct Token<'a> {
    pub filepath: &'a str,
    pub row: usize,
    pub col: usize,
    pub value: &'a str,
}

impl<'a> Token<'a> {
    pub fn new(filepath: &'a str, row: usize, col: usize, value: &'a str) -> Token<'a> {
        Token {
            filepath,
            row,
            col,
            value,
        }
    }
}

pub fn tokenize_line<'a>(filepath: &'a str, line_number: usize, source: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = vec![];

    let mut token_start = 0;
    let mut moving_start = true;
    let mut reading_string = false;

    for (col, ch) in source.char_indices() {
        if moving_start {
            if col != source.len() - 1 {
                if &source[col..(col + 2)] == "//" {
                    return tokens;
                }
            }
            if !ch.is_whitespace() {
                token_start = col;
                moving_start = false;
                if ch == '"' {
                    reading_string = true;
                }
            }
        } else {
            if reading_string {
                if ch == '"' {
                    let token = Token::new(
                        filepath,
                        line_number,
                        token_start + 1,
                        &source[token_start..(col + 1)],
                    );
                    tokens.push(token);
                    moving_start = true;
                    reading_string = false;
                }
            } else if ch.is_whitespace() {
                let token = Token::new(
                    filepath,
                    line_number,
                    token_start + 1,
                    &source[token_start..col],
                );
                tokens.push(token);
                moving_start = true;
            }
        }
        if col == source.len() - 1 && !ch.is_whitespace() && ch != '"' {
            let token = Token::new(
                filepath,
                line_number,
                token_start + 1,
                &source[token_start..(col + 1)],
            );
            tokens.push(token);
        }
    }
    tokens
}

pub fn tokenize_source_code<'a>(filepath: &'a str, source: &'a String) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = vec![];

    let mut line_number = 1;
    for line in source.lines() {
        let temp_tokens = tokenize_line(filepath, line_number, line);
        for token in temp_tokens {
            tokens.push(token);
        }
        line_number += 1;
    }
    tokens
}
