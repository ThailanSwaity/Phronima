pub struct Lexer {
    pub source_code: String,
    cursor: u_size,
    tokens: Vec<&str>, 
}

impl Lexer {
    pub fn new(source_code: String) -> Lexer {
        Lexer { source_code, cursor: 0, tokens: vec![] }
    }

    pub fn tokenize(&mut self) {
        let bytes = self.source_code.as_bytes();

        let mut token = "";

        for (i, &char) in bytes.iter().enumerate() {
            if item == b' ' {
                if token.len() > 0 {
                    self.tokens.push(token.clone());
                    token = "";
                } 
            }
            else {
                token.push(*char); 
            }
        }
    }

    pub fn next_token(&mut self) -> &str {
        let mut token = String::new();
        let source_as_chars = self.source_code.chars();
        
        while cursor < source_as_chars.len() {
            let char = source_code.
        }
    }
}
