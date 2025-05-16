use std::fs;
use std::fs::File;
use std::io::Write;
use std::env;
use std::process;
use std::error::Error;
use phronima::{ Stack, Function };

fn main() {
    let args: Vec<String> = env::args().collect();

    let filepath = "./helloworld.bf";
    let program: Vec<Function> = read_program_from_file(&filepath).unwrap_or_else(|err| {
        eprintln!("Application error: {err}");
        process::exit(1);
    });

    if &args[1] == "sim" {
        simulate_program(program);
    }
    else if &args[1] == "com" {
        let compiled_code = compile_program(program).unwrap_or_else(|err| {
            eprintln!("Application error: {err}");
            process::exit(1);
        });
        let _ = write_program_to_file(compiled_code);
    }
    else {
        eprintln!("Must state whether to compile 'com' or simulate 'sim' the program");
        process::exit(1);
    }
}

fn read_program_from_file(filepath: &str) -> Result<Vec<Function>, Box<dyn Error>> {
    let source = fs::read_to_string(filepath)?;
    let tokens = phronima::tokenize(&source);
    let parsed_tokens = phronima::parse_tokens(tokens)?;
    let program = phronima::create_references_for_blocks(parsed_tokens)?;
    Ok(program)
}

fn write_program_to_file(compiled_code: String) -> Result<(), Box<dyn Error>> {
    let mut file = File::create("program_comp.txt")?;
    file.write_all(&compiled_code.as_bytes())?;
    Ok(())
}

fn compile_program(program: Vec<Function>) -> Result<String, Box<dyn Error>> {
    let mut compiled_code: String = String::from("");

    // I'm using stack and memory here to assist with memory usage in brainf*ck
    // I have not bothered to try and write an implementation that would allow run-time memory usage
    const stack_start: usize = 256;
    let mut stack: Stack = Stack::new();
    let mut memory: [u8; stack_start] = [0u8; stack_start];

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
                // compile-time for printing strings
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
                compiled_code.push('[');
                compiled_code.push('-');
                compiled_code.push(']');
            },
            Function::End(_index) => {
                compiled_code.push('>');
                compiled_code.push(']');
                compiled_code.push('<');
            },
            Function::Else(_index) => {
                todo!("else compiler code");
            },
            Function::While(_index) => {
                todo!("while compiler code");
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
                todo!("swap compiler code");
            },
            Function::Dup() => {
                todo!("dup compiler code");
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
                if a == 0 {
                    i = index.unwrap();
                    continue;
                } else {
                    stack.push(a);
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
