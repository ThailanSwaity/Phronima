use std::fs;
use std::fs::File;
use std::io::Write;
use std::io;
use std::env;
use std::process;
use std::error::Error;
use phronima::{ Stack, Function };

fn main() {
    let args: Vec<String> = env::args().collect();


    if &args[1] == "sim" || &args[1] == "com" {
        check_args(args.len());
        let filepath = &args[2];
        let program: Vec<Function> = read_program_from_file(filepath).unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            process::exit(1);
        });
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

fn read_program_from_file(filepath: &str) -> Result<Vec<Function>, Box<dyn Error>> {
    let source = fs::read_to_string(filepath)?;
    let tokens = phronima::tokenize_source_code(filepath, &source);
    let parsed_tokens = phronima::parse_tokens(tokens)?;
    let program = phronima::create_references_for_blocks(parsed_tokens)?;
    Ok(program)
}

fn compile_program_from_file(filepath: &str) -> Result<String, Box<dyn Error>> {
    let source = fs::read_to_string(filepath)?;
    let bf_code = compile_program_from_source(filepath, source)?;
    Ok(bf_code)
}

fn compile_program_from_source(filepath: &str, source_code: String) -> Result<String, Box<dyn Error>> {
    let tokens = phronima::tokenize_source_code(filepath, &source_code);
    let parsed_tokens = phronima::parse_tokens(tokens)?;
    let program = phronima::create_references_for_blocks(parsed_tokens)?;
    let bf_code = compile_program(program)?;
    Ok(bf_code)
}

fn write_program_to_file(filepath: &str, compiled_code: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create(filepath)?;
    file.write_all(&compiled_code.as_bytes())?;
    Ok(())
}

fn compile_program(program: Vec<Function>) -> Result<String, Box<dyn Error>> {
    let mut compiled_code: String = String::from("");

    // I'm using stack and memory here to assist with memory usage in brainf*ck
    // I have not bothered to try and write an implementation that would allow run-time memory usage
    const STACK_START: usize = 256;
    let mut stack: Stack = Stack::new();
    let mut memory: [u8; STACK_START] = [0u8; STACK_START];

    for _i in 0..255 {
        compiled_code.push('>');
    }

    for i in 0..program.len() {
        match program[i] {
            Function::Push(byte) => {
                compiled_code.push('>');
                for _i in 0..byte {
                    compiled_code.push('+');
                }

                stack.push(byte);
            },
            Function::Pop() => {
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');

                stack.pop();
            },
            Function::Plus() => {
                compiled_code.push('[');
                compiled_code.push('<');
                compiled_code.push('+');
                compiled_code.push('>');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');

                let a = stack.pop();
                let b = stack.pop();
                stack.push(a + b);
            },
            Function::Minus() => {
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('<');
                compiled_code.push('-');
                compiled_code.push('>');
                compiled_code.push(']');

                compiled_code.push('<');

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
                compiled_code.push('.');
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');

                stack.pop();
            },
            Function::NumOut() => {
                todo!("numout compiler code");
            },
            Function::Write() => {

                // We do not use the brainf*ck memory for run-time. But we can use it for
                // compile-time and for printing strings
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');
                let byte = stack.pop();

                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');
                let addr = stack.pop();

                for _i in 0..(255 + stack.top - addr as usize) {
                    compiled_code.push('<');
                }

                // We know the value at this address but it is still more concise to write it this way
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');

                for _i in 0..byte {
                    compiled_code.push('+');
                }
                for _i in 0..(255 + stack.top - addr as usize) {
                    compiled_code.push('>');
                }

                memory[addr as usize] = byte;
            },
            Function::Read() => {
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
                compiled_code.push('<');
                let addr = stack.pop();

                let byte = memory[addr as usize];

                compiled_code.push('>');
                for _i in 0..byte {
                    compiled_code.push('+');
                }
                stack.push(byte);
            },
            Function::Mem() => {
                compiled_code.push('>');
                stack.push(0u8);
            },
            Function::If(_index) => {
                compiled_code.push('[');
            },
            Function::End(index) => {
                if index.unwrap() == program.len() {
                    compiled_code.push(']');
                }
                else if index.unwrap() > program.len() {
                    eprintln!("Something really not great happened here...");
                    process::exit(1);
                }
                else {
                    match program[index.unwrap()] {
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
            Function::Else(_index) => {
                todo!("else compiler code");
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
                compiled_code.push('<');
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('>');
                compiled_code.push('>');
                compiled_code.push('+');
                compiled_code.push('<');
                compiled_code.push('<');
                compiled_code.push(']');

                compiled_code.push('>');

                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('<');
                compiled_code.push('+');
                compiled_code.push('>');
                compiled_code.push(']');

                compiled_code.push('>');

                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('<');
                compiled_code.push('+');
                compiled_code.push('>');
                compiled_code.push(']');

                compiled_code.push('<');

                let a = stack.pop();
                let b = stack.pop();
                stack.push(a);
                stack.push(b);
            },
            Function::Dup() => {
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('>');
                compiled_code.push('+');
                compiled_code.push('>');
                compiled_code.push('+');
                compiled_code.push('<');
                compiled_code.push('<');
                compiled_code.push(']');

                compiled_code.push('>');
                compiled_code.push('>');
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push('<');
                compiled_code.push('<');
                compiled_code.push('+');
                compiled_code.push('>');
                compiled_code.push('>');
                compiled_code.push(']');
                compiled_code.push('<');

                let a = stack.pop();
                stack.push(a);
                stack.push(a);
            }
        }
    }
    Ok(compiled_code)
}

fn simulate_program(program: Vec<Function>) {
    let mut stack: Stack = Stack::new();

    let mut memory: [u8; 256] = [0u8; 256];

    let mut i = 0;
    while i  < program.len() {
        match program[i] {
            Function::Push(byte) => {
                stack.push(byte);
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
                stack.push(0u8);
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
            }
        }
        i += 1;
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
}
