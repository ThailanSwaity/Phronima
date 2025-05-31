use std::fs;
use std::fs::File;
use std::io::Write;
use std::io;
use std::env;
use std::process;
use std::error::Error;
use phronima::{ Stack, Function, Program };

fn main() {
    let args: Vec<String> = env::args().collect();


    if &args[1] == "sim" || &args[1] == "com" {
        check_args(args.len());
        let filepath = &args[2];
        let program: Program = read_program_from_file(filepath).unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            process::exit(1);
        });
        if !program.functions.contains_key("main") {
            eprintln!("Could not find function main");
            process::exit(1);
        }
        if &args[1] == "sim" {
            simulate_program(program);
        }
        else if &args[1] == "com" {
            check_args(args.len());
            let compiled_code = compile_program(program).unwrap_or_else(|err| {
                eprintln!("Application error: {err}");
                process::exit(1);
            });
            let _ = write_program_to_file("compiled_code.txt", compiled_code);
        }
    }
    else if &args[1] == "rec"{
        println!("Creating compilatin test validation files...\n");
        let _ = record_for_test();
        println!("\ncomplete.");
    }
    else {
        eprintln!("Must state whether to compile 'com' or simulate 'sim' the program");
        process::exit(1);
    }
}

fn check_args(num_args: usize) {
    if num_args < 3 {
        eprintln!("Must provide 2 arguments\n['sim', 'com'] and 'filepath'");
        process::exit(1);
    }
}

fn read_program_from_file(filepath: &str) -> Result<Program, Box<dyn Error>> {
    let source = fs::read_to_string(filepath)?;
    let tokens = phronima::tokenize_source_code(filepath, &source);
    let parsed_tokens = phronima::parse_tokens(tokens)?;
    let mut program = phronima::parse_program_structure(parsed_tokens)?;
    let _ = handle_imports(&mut program);
    for (_fname, fblock) in &mut program.functions {
        phronima::create_references_for_blocks(fblock);
    }
    Ok(program)
}

fn handle_imports(program: &mut Program) -> Result<(), Box<dyn Error>> {
    while let Some(filepath) = program.imports.pop_front() {
        let source = fs::read_to_string(&filepath)?;
        let tokens = phronima::tokenize_source_code(&filepath, &source);
        let parsed_tokens = phronima::parse_tokens(tokens)?;
        let import_program = phronima::parse_program_structure(parsed_tokens)?;
        program.consume(import_program);
    }
    Ok(())
}

fn compile_program_from_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let source = fs::read_to_string(filepath)?;
    let bf_code = compile_program_from_source(filepath, source)?;
    Ok(bf_code)
}

fn compile_program_from_source(filepath: &str, source_code: String) -> Result<String, Box<dyn Error>> {
    let tokens = phronima::tokenize_source_code(filepath, &source_code);
    let parsed_tokens = phronima::parse_tokens(tokens)?;
    let mut program = phronima::parse_program_structure(parsed_tokens)?;
    for (_fname, fblock) in &mut program.functions {
        phronima::create_references_for_blocks(fblock);
    }
    let bf_code = compile_program(program)?;
    Ok(bf_code)
}

fn write_program_to_file(filepath: &str, compiled_code: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filepath)?;
    file.write_all(&compiled_code.as_bytes())?;
    Ok(())
}

fn compile_program(program: Program) -> Result<String, Box<dyn Error>> {
    let mut compiled_code: String = String::from("");
    let program = program.functions;

    let mut call_stack: Vec<(String, usize)> = vec![];
    let mut current_function = program.get("main").unwrap();
    let mut current_function_name: String = "main".to_string();

    // I'm using stack and memory here to assist with memory usage in brainf*ck
    // I have not bothered to try and write an implementation that would allow run-time memory usage
    const STACK_START: usize = 256;
    let mut stack: Stack = Stack::new();
    let mut memory: [u8; STACK_START] = [0u8; STACK_START];

    for _i in 0..255 {
        compiled_code.push('>');
    }

    let mut i = 0;
    while i < current_function.len() {
        match &current_function[i] {
            Function::Push(byte) => {
                compiled_code.push('>');
                for _i in 0..*byte {
                    compiled_code.push('+');
                }

                stack.push(*byte);
            },
            Function::Pop() => {
                compiled_code.push_str("[-]<");

                stack.pop();
            },
            Function::Plus() => {
                compiled_code.push_str("[<+>-]<");

                let a = stack.pop();
                let b = stack.pop();
                stack.push(a + b);
            },
            Function::Minus() => {
                compiled_code.push_str("[-<->]<");

                let b = stack.pop();
                let a = stack.pop();
                stack.push(a - b);
            },
            Function::Mult() => {
                compiled_code.push_str("<[->>+<<]>[->[->+<<<+>>]>[-<+>]<<]>[-]<<");

                let b = stack.pop();
                let a = stack.pop();
                stack.push(a * b);
            },
            Function::CharOut() => {
                compiled_code.push_str(".[-]<");

                stack.pop();
            },
            Function::NumOut() => {
                todo!("numout compiler code");
            },
            Function::Write() => {
                // As a better alternative, we want the address pointer to be constant during loops instead
                // of the address value
                
                // So while popping the byte value, we want to copy it to the address in memory
                // instead of hardcoding the value
                let byte = stack.pop();

                let addr = stack.pop();

                // In the brainfuck code, the address and byte haven't been popped yet
                let stack_top: usize = stack.top + 2;

                // First we have to set the value at the address to 0
                for _i in 0..(255 + stack_top - addr as usize) {
                    compiled_code.push('<');
                }
                compiled_code.push_str("[-]");
                for _i in 0..(255 + stack_top - addr as usize) {
                    compiled_code.push('>');
                }

                // This copies the value from the top of the stack to the memory address
                compiled_code.push_str("[-");
                for _i in 0..(255 + stack_top - addr as usize) {
                    compiled_code.push('<');
                }
                compiled_code.push('+');
                for _i in 0..(255 + stack_top - addr as usize) {
                    compiled_code.push('>');
                }
                compiled_code.push_str("]<");

                // This pops the address from the stack, we do not need to copy the address pointer
                compiled_code.push_str("[-]<");

                memory[addr as usize] = byte;
            },
            Function::Read() => {
                // The read must copy the value into two places first

                compiled_code.push_str("[-]<");
                let addr = stack.pop();

                let byte = memory[addr as usize];

                compiled_code.push('>');
                stack.push(byte);

                for _i in 0..(255 + stack.top - addr as usize) {
                    compiled_code.push('<');
                }
                compiled_code.push_str("[-");
                for _i in 0..(255 + stack.top - addr as usize) {
                    compiled_code.push('>');
                }
                compiled_code.push('+');
                for _i in 0..(255 + stack.top) {
                    compiled_code.push('<');
                }
                compiled_code.push('+');
                for _i in 0..(addr as usize) {
                    compiled_code.push('>');
                }
                compiled_code.push(']');
                
                // Finished copying value onto stack. We now have to copy the value back into the
                // original address
                for _i in 0..(addr as usize) {
                    compiled_code.push('<');
                }
                compiled_code.push_str("[-");
                for _i in 0..(addr as usize) {
                    compiled_code.push('>');
                }
                compiled_code.push('+');
                for _i in 0..(addr as usize) {
                    compiled_code.push('<');
                }
                compiled_code.push(']');

                // Go back to top of stack
                for _i in 0..(255 + stack.top) {
                    compiled_code.push('>');
                }
            },
            Function::Mem() => {
                compiled_code.push_str(">+");
                stack.push(1u8);
            },
            Function::If(_index) => {
                compiled_code.push('[');
            },
            Function::End(index) => {
                if index.unwrap() == current_function.len() {
                    compiled_code.push_str(">]<");
                }
                else if index.unwrap() > current_function.len() {
                    eprintln!("Something really not great happened here...");
                    process::exit(1);
                }
                else {
                    match current_function[index.unwrap()] {
                        Function::While(_index) => {
                            compiled_code.push(']');
                        },
                        _ => {
                            compiled_code.push_str(">]<"); // This moves the cell pointer to a block
                            // with a value of 0 to ensure the if block never executes more than once
                        },
                    }
                }
            },
            // TODO: fix the behaviour of if and else incorrectly moving the stack
            Function::Else(_index) => {
                compiled_code.push_str(">]<");

                compiled_code.push_str(">[-]<-[>-<-]>[<+>-]<");
                let byte = stack.pop();

                let not_byte = 1u8.wrapping_sub(byte);
                stack.push(not_byte);

                compiled_code.push('[');
            },
            Function::While(_index) => {
                compiled_code.push_str("[");
            }
            Function::LessThan() => {
                todo!("lessthan compiler code");
            },
            Function::GreaterThan() => {
                todo!("greaterthan compiler code");
            },
            Function::Equals() => {
                todo!("equals compiler code");
            },
            Function::Swap() => {
                compiled_code.push_str("<[->>+<<]>[-<+>]>[-<+>]<");

                let a = stack.pop();
                let b = stack.pop();
                stack.push(a);
                stack.push(b);
            },
            Function::Dup() => {
                compiled_code.push_str("[->+>+<<]>>[-<<+>>]<");

                let a = stack.pop();
                stack.push(a);
                stack.push(a);
            },
            Function::Not() => {
                compiled_code.push_str(">[-]<-[>-<-]>[<+>-]<");
                let byte = stack.pop();

                let not_byte = 1u8.wrapping_sub(byte);
                stack.push(not_byte);
            },
            Function::FunctionDeclaration(_) => {
                println!("This shouldn't be reachable");
            },
            // There's definitely a better way to do this
            Function::FunctionCall(function_name) => {
                call_stack.push((current_function_name.clone(), i));
                i = 0;
                current_function = program.get(&function_name.clone()).unwrap_or_else(|| {
                    eprintln!("Unknown function: {}", function_name);
                    process::exit(1);
                });
                current_function_name = function_name.clone();
                continue;
            },
            Function::StringLiteral(string_literal) => {
                let byte_string = string_literal.as_bytes();

                // Push 0 (NULL character) to the stack
                compiled_code.push('>');
                stack.push(0u8);

                // Push each character in the string to the stack in reverse order
                for i in (0..byte_string.len()).rev() {
                    stack.push(byte_string[i]);
                    compiled_code.push('>');
                    for _i in 0..byte_string[i] {
                        compiled_code.push('+');
                    }
                }
            },
            Function::Import(_) => {
                eprintln!("Unreachable");
            },
        }
        i += 1;
        // and this as well
        if i == current_function.len() && (&current_function_name != "main") {
            let (fname, index) = call_stack.pop().unwrap();
            i = index + 1;
            current_function_name = fname.clone();
            current_function = program.get(&fname).unwrap();
        }
    }
    Ok(compiled_code)
}

fn simulate_program(program: Program) {
    let program = program.functions;

    let mut stack: Stack = Stack::new();
    let mut call_stack: Vec<(String, usize)> = vec![];
    let mut current_function = program.get("main").unwrap();
    let mut current_function_name: String = "main".to_string();

    let mut memory: [u8; 256] = [0u8; 256];

    let mut i = 0;
    while i < current_function.len() {
        match &current_function[i] {
            Function::Push(byte) => {
                stack.push(*byte);
            },
            Function::Pop() => {
                stack.pop();
            },
            Function::Plus() => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(a + b);
            },
            Function::Minus() => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a - b);
            },
            Function::Mult() => {
                let b = stack.pop();
                let a = stack.pop();
                stack.push(a * b);
            },
            Function::CharOut() => {
                print!("{}", stack.pop() as char);
            },
            Function::NumOut() => {
                print!("{}", stack.pop());
            },
            Function::Write() => {
                let a = stack.pop();
                let b = stack.pop();
                memory[b as usize] = a;
            },
            Function::Read() => {
                let a = stack.pop();
                stack.push(memory[a as usize]);
            },
            Function::Mem() => {
                stack.push(1u8);
            },
            Function::If(index) => {
                let a = stack.pop();
                stack.push(a);
                if a == 0 {
                    i = index.unwrap();
                    continue;
                }
            },
            Function::End(index) => {
                i = index.unwrap();
                continue;
            },
            Function::Else(index) => {
                i = index.unwrap();
                continue;
            },
            Function::While(index) => {
                let a = stack.pop();
                stack.push(a);
                if a == 0 {
                    i = index.unwrap();
                    continue;
                }
            }
            Function::LessThan() => {
                let b = stack.pop();
                let a = stack.pop();
                if a < b {
                    stack.push(1);
                }
                else {
                    stack.push(0);
                }
            },
            Function::GreaterThan() => {
                let b = stack.pop();
                let a = stack.pop();
                if a > b {
                    stack.push(1);
                }
                else {
                    stack.push(0);
                }
            },
            Function::Equals() => {
                let b = stack.pop();
                let a = stack.pop();
                if a == b {
                    stack.push(1);
                }
                else {
                    stack.push(0);
                }
            },
            Function::Swap() => {
                let a = stack.pop();
                let b = stack.pop();
                stack.push(a);
                stack.push(b);
            },
            Function::Dup() => {
                let a = stack.pop();
                stack.push(a);
                stack.push(a);
            },
            Function::Not() => {
                let byte = stack.pop();

                let not_byte = 1u8.wrapping_sub(byte);
                stack.push(not_byte);
            },
            Function::FunctionDeclaration(_) => {
                println!("This shouldn't be reachable");
            },
            // There's definitely a better way to do this
            Function::FunctionCall(function_name) => {
                call_stack.push((current_function_name.clone(), i));
                i = 0;
                current_function = program.get(&function_name.clone()).unwrap_or_else(|| {
                    eprintln!("Unknown function: {}", function_name);
                    process::exit(1);
                });
                current_function_name = function_name.clone();
                continue;
            },
            Function::StringLiteral(string_literal) => {
                let byte_string = string_literal.as_bytes();
                stack.push(0u8);
                for i in (0..byte_string.len()).rev() {
                    stack.push(byte_string[i]);
                }
            },
            Function::Import(_) => {
                eprintln!("Unreachable");
            },
        }
        i += 1;
        // and this as well
        if i == current_function.len() && (&current_function_name != "main") {
            let (fname, index) = call_stack.pop().unwrap();
            i = index + 1;
            current_function_name = fname.clone();
            current_function = program.get(&fname).unwrap();
        }
    }
}

// I don't care to catch each error here.
// TODO: fix this mess
fn record_for_test() -> io::Result<()> {
    for entry in fs::read_dir("./tests/")? {
        let dir = entry?;
        let mut path = dir.path();
        let extension = path.extension().unwrap();
        if extension.to_str().unwrap() == "phron" {
            println!("{:?}", dir.path());
            println!("{}", path.to_str().unwrap());
            let bf_code = compile_program_from_file(path.to_str().unwrap()).unwrap();
            path.set_extension("bf");
            let _ = write_program_to_file(path.to_str().unwrap(), bf_code);
            println!("{}", path.to_str().unwrap());
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    // https://blog.v-gar.de/2019/04/rust-remove-trailing-newline-after-input/
    fn trim_newline(s: &mut String) {
        while s.ends_with('\n') || s.ends_with('\r') {
            s.pop();
        }
    }

    fn make_code_for_comparison(validation_code_filepath: &str, file_to_compile: &str) -> (String, String) {
        let mut validation_bf_code = fs::read_to_string(validation_code_filepath).unwrap();
        trim_newline(&mut validation_bf_code);
        let compiled_bf_code = compile_program_from_file(file_to_compile).unwrap();
        (validation_bf_code, compiled_bf_code)
    }

    fn test(operation_name: &str) -> bool {
        let mut valid_code_path = PathBuf::new();
        let mut test_code_path = PathBuf::new();

        valid_code_path.push("./tests/");
        valid_code_path.push(operation_name);
        valid_code_path.set_extension("bf");

        test_code_path.push("./tests/");
        test_code_path.push(operation_name);
        test_code_path.set_extension("phron");

        let (valid_code, compiled_code) = make_code_for_comparison(valid_code_path.to_str().unwrap(), test_code_path.to_str().unwrap());
        valid_code == compiled_code
    }

    #[test]
    fn push_op() {
        assert!(test("push"));
    }

    #[test]
    fn pop_op() {
        assert!(test("pop"));
    }

    #[test]
    fn addition_op() {
        assert!(test("add"));
    }

    #[test]
    fn subtraction_op() {
        assert!(test("sub"));
    }

    #[test]
    fn multiplication_op() {
        assert!(test("mult"));
    }

    #[test]
    fn chout_op() {
        assert!(test("chout"));
    }

    #[test]
    fn write_op() {
        assert!(test("write"));
    }

    #[test]
    fn read_op() {
        assert!(test("read"));
    }

    #[test]
    fn mem_op() {
        assert!(test("mem"));
    }

    #[test]
    fn if_op() {
        assert!(test("if"));
    }

    #[test]
    fn while_loop() {
        assert!(test("while"));
    }

    #[test]
    fn swap_op() {
        assert!(test("swap"));
    }

    #[test]
    fn dup_op() {
        assert!(test("dup"));
    }

    #[test]
    fn else_op() {
        assert!(test("else"));
    }

    #[test]
    fn not_op() {
        assert!(test("not"));
    }
}
