use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
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
    If(Option<usize>),
    End(Option<usize>),
    Else(Option<usize>),
    While(Option<usize>),
    LessThan(),
    GreaterThan(),
    Equals(),
    Swap(),
    Dup(),
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

    for token in tokens.iter() {
        if let Ok(number) = token.value.parse::<u8>() {
            parsed_tokens.push(Function::Push(number));
        }
        else if token.value == "pop" {
            parsed_tokens.push(Function::Pop());
        }
        else if token.value == "+" {
            parsed_tokens.push(Function::Plus());
        }
        else if token.value == "-" {
            parsed_tokens.push(Function::Minus());
        }
        else if token.value == "*" {
            parsed_tokens.push(Function::Mult());
        }
        else if token.value == "chout" {
            parsed_tokens.push(Function::CharOut());
        }
        else if token.value == "numout" {
            parsed_tokens.push(Function::NumOut());
        }
        else if token.value == "write" {
            parsed_tokens.push(Function::Write());
        }
        else if token.value == "read" {
            parsed_tokens.push(Function::Read());
        }
        else if token.value == "mem" {
            parsed_tokens.push(Function::Mem());
        }
        else if token.value == "if" {
            parsed_tokens.push(Function::If(None));
        }
        else if token.value == "end" {
            parsed_tokens.push(Function::End(None));
        }
        else if token.value == "else" {
            parsed_tokens.push(Function::Else(None));
        }
        else if token.value == "while" {
            parsed_tokens.push(Function::While(None));
        }
        else if token.value == "<" {
            parsed_tokens.push(Function::LessThan());
        }
        else if token.value == ">" {
            parsed_tokens.push(Function::GreaterThan());
        }
        else if token.value == "=" {
            parsed_tokens.push(Function::Equals());
        }
        else if token.value == "swap" {
            parsed_tokens.push(Function::Swap());
        }
        else if token.value == "dup" {
            parsed_tokens.push(Function::Dup());
        }
        else {
            eprintln!("{}:{}:{} could not parse token: '{}'",
                token.filepath,
                token.row,
                token.col,
                token.value
            ); // Need to improve better error reporting
            return Err("Syntax error")?;
        }
    }
    Ok(parsed_tokens)
}

pub fn create_references_for_blocks(parsed_tokens: Vec<Function>) -> Result<Vec<Function>, Box<dyn Error>> {
    let mut block_tokens: Vec<(usize, u8)> = vec![];
    let mut parsed_tokens = parsed_tokens;

    const IF: u8    = 0;
    const ELSE: u8  = 1;
    const WHILE: u8 = 2;

    for i in 0..parsed_tokens.len() {
        match parsed_tokens[i] {
            Function::If(_reference) => {
                block_tokens.push((i, IF)); 
            },
            Function::Else(_reference) => {
                let (index, block_word_type) = block_tokens[block_tokens.len() - 1];
                if block_word_type == IF {
                    parsed_tokens[index] = Function::If(Some(i + 1));
                    block_tokens.push((i, ELSE));
                }
            },
            Function::End(_reference) => {
                let (index, block_word_type) = block_tokens[block_tokens.len() - 1];
                if block_word_type == IF {
                    parsed_tokens[index] = Function::If(Some(i));
                    parsed_tokens[i] = Function::End(Some(i + 1));
                    let _tk = block_tokens.pop();
                }
                else if block_word_type == ELSE {
                    parsed_tokens[index] = Function::Else(Some(i));
                    parsed_tokens[i] = Function::End(Some(i + 1));
                    let _tk = block_tokens.pop();
                    let _tk = block_tokens.pop();
                }
                else if block_word_type == WHILE {
                    parsed_tokens[index] = Function::While(Some(i + 1));
                    parsed_tokens[i] = Function::End(Some(index));
                    let _tk = block_tokens.pop();
                }
            },
            Function::While(_reference) => {
                block_tokens.push((i, WHILE));
            }
            _ => {}
        }
    }
    Ok(parsed_tokens)
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
        Token { filepath, row, col, value }
    }
}

pub fn tokenize_line<'a>(filepath: &'a str, line_number: usize, source: &'a str) -> Vec<Token<'a>> {
    let mut tokens: Vec<Token> = vec![];

    let mut token_start = 0;
    let mut moving_start = true;

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
            }
        }
        else {
            if ch.is_whitespace() {
                let token = Token::new(filepath, line_number, token_start + 1, &source[token_start..col]);
                tokens.push(token);
                moving_start = true;
            }
        }
        if col == source.len() - 1 && !ch.is_whitespace() {
            let token = Token::new(filepath, line_number, token_start + 1, &source[token_start..(col + 1)]);
            tokens.push(token);
        }
    }
    tokens
}

pub fn tokenize_file<'a>(filepath: &'a str, source: &'a String) -> Vec<Token<'a>> {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_test() {
        let test_source = "  34 43    67".to_string();

        let filepath = "file.test";
        let tokens = tokenize_file(&filepath, &test_source);
        assert_eq!(vec!["34", "43", "67"], tokens);
    }

    #[test]
    fn parse_tokens_test() {
        let test_tokens = vec!["34", "43", "67", "+", "numout"];

        let parsed_tokens = parse_tokens(test_tokens);
        assert_eq!(vec![Function::Push(34u8), Function::Push(43u8), Function::Push(67u8), Function::Plus(), Function::NumOut()], parsed_tokens);
    }
}
