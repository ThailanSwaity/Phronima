use std::error::Error;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Function {
    Push(u8),
    Pop(),
    Plus(),
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

pub fn parse_tokens(tokens: Vec<&str>) -> Result<Vec<Function>, Box<dyn Error>> {
    let mut parsed_tokens: Vec<Function> = vec![];

    for token in tokens.iter() {
        if let Ok(number) = token.parse::<u8>() {
            parsed_tokens.push(Function::Push(number));
        }
        else if *token == "pop" {
            parsed_tokens.push(Function::Pop());
        }
        else if *token == "+" {
            parsed_tokens.push(Function::Plus());
        }
        else if *token == "chout" {
            parsed_tokens.push(Function::CharOut());
        }
        else if *token == "numout" {
            parsed_tokens.push(Function::NumOut());
        }
        else if *token == "write" {
            parsed_tokens.push(Function::Write());
        }
        else if *token == "read" {
            parsed_tokens.push(Function::Read());
        }
        else if *token == "mem" {
            parsed_tokens.push(Function::Mem());
        }
        else if *token == "if" {
            parsed_tokens.push(Function::If(None));
        }
        else if *token == "end" {
            parsed_tokens.push(Function::End(None));
        }
        else if *token == "else" {
            parsed_tokens.push(Function::Else(None));
        }
        else if *token == "while" {
            parsed_tokens.push(Function::While(None));
        }
        else if *token == "<" {
            parsed_tokens.push(Function::LessThan());
        }
        else if *token == ">" {
            parsed_tokens.push(Function::GreaterThan());
        }
        else if *token == "=" {
            parsed_tokens.push(Function::Equals());
        }
        else if *token == "swap" {
            parsed_tokens.push(Function::Swap());
        }
        else if *token == "dup" {
            parsed_tokens.push(Function::Dup());
        }
        else {
            return Err("Could not parse token '{token}'")?; // Need to improve better error reporting
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

pub fn tokenize(source: &String) -> Vec<&str> {
    let mut tokens: Vec<&str> = vec![];

    let mut token_start = 0;
    let mut moving_start = true;

    for (i, ch) in source.char_indices() {
        if moving_start {
            if !ch.is_whitespace() {
                token_start = i;
                moving_start = false;
            }
        }
        else {
            if ch.is_whitespace() {
                tokens.push(&source[token_start..i]);
                moving_start = true;
            }
        }
        if i == source.len() - 1 && ch.is_alphanumeric() {
            tokens.push(&source[token_start..(i + 1)]);
        }
    }
    tokens
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_test() {
        let test_source = "  34 43    67".to_string();

        let tokens = tokenize(&test_source);
        assert_eq!(vec!["34", "43", "67"], tokens);
    }

    #[test]
    fn parse_tokens_test() {
        let test_tokens = vec!["34", "43", "67", "+", "numout"];

        let parsed_tokens = parse_tokens(test_tokens);
        assert_eq!(vec![Function::Push(34u8), Function::Push(43u8), Function::Push(67u8), Function::Plus(), Function::NumOut()], parsed_tokens);
    }
}
