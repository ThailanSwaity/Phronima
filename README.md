# Phronima
Phronima is a stack-based high(er)-level language for brainf*ck

# Intro 
What is brainf*ck? [Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) is an esoteric programming language modelled after the [Turing Machine](https://en.wikipedia.org/wiki/Turing_machine)
# Goals 
- [x] [Hello, world!](./examples/helloworld.phron)
- [x] ([if, end](./examples/if.phron), [while](./examples/while.phron)) Control flow operators (if, else, while)
- [ ] Character input, number input, string input
- [ ] Named pointers (variables)
- [x] Verbose code syntax errors
- [x] Comments
- [ ] Strings
- [ ] Optimize compiled brainf*ck
- [ ] Turing completeness
- [ ] Interpreter with graphics library (will be separate project)

# Modes 
There are two modes/subcommands
  'com': will compile the source code into brainf*ck code
  'sim': will simulate the program created by the source code 

NOTE: some operations do not currently work in 'compile' mode. Still a work in progress.

      cargo run -- sim ./helloworld.phron
      cargo run -- com ./helloworld.phron

Currently, the compiled code will be written to a file called "program_comp.txt"

# Memory
Phronima uses an array of 256 bytes as its addressable "memory", with the remaining 29,744 cells of brainf*ck to be used as a stack

There is no type system implemented and I only plan on having two: byte and pointer.
Be careful when writing with this language as it is very easy to cause stack underflow

Memory can be read during run-time through the use of static compile-time pointers.
The address assigned to a read or write operation cannot change at run-time (this will lead to undefined behaviour of the brainf*ck program), but the byte value can (this was previously the other way around, but I think this is much better)

# Supported operations
| operation|compiler   |simulator  |
|------------|-----------|-----------|
| push  |:heavy_check_mark: | :heavy_check_mark: |
| pop   |:heavy_check_mark: | :heavy_check_mark:|
| + (addition)  |:heavy_check_mark: |:heavy_check_mark: |
| - (subtraction)|:heavy_check_mark: |:heavy_check_mark: |
| * (multiplication) |:heavy_check_mark:|:heavy_check_mark:|
| modulo| | |
| chout |:heavy_check_mark: |:heavy_check_mark: |
| numout| |:heavy_check_mark: |
| write |:heavy_check_mark: |:heavy_check_mark: |
| read  |:heavy_check_mark: |:heavy_check_mark: |
| mem   |:heavy_check_mark: |:heavy_check_mark: |
| if    |:heavy_check_mark: |:heavy_check_mark: |
| end   |:heavy_check_mark: |:heavy_check_mark: |
| else  | |:heavy_check_mark: |
| while |:heavy_check_mark: |:heavy_check_mark: |
| < (less than)    | |:heavy_check_mark: |
| > (greater than)    | |:heavy_check_mark: |
| = (equal to)    | |:heavy_check_mark: |
| swap  |:heavy_check_mark: |:heavy_check_mark: |
| dup   |:heavy_check_mark: |:heavy_check_mark: |

# Operation descriptions

## Stack
| |Stack Behaviour|
|-|-----------|
| push| a -> a b|
| pop | a b -> a|
| dup | a -> a a |
| swap| a b -> b a|

## Math 
| |Stack Behaviour|
|-|-----------|
| +| a b -> (a+b) |
| -| a b -> (a-b) |
| *| a b -> (a*b) |
| >| a b -> 1 if a > b else 0  |
| <| a b -> 1 if a < b else 0|
| =| a b -> 1 if a = b else 0|

## Memory
| |Stack Behaviour|
|-|-----------|
| read| a -> pops a from the stack, pushes byte at memory address a to the stack |
| write| a b -> pops a and b from the stack, writes b to address a |

## Control flow
| |Stack Behaviour|
|-|-----------|
| if, else, end| a -> reads top of the stack, executes if block if a > 0, executes else block if a = 0|
|while, end| a -> reads top of the stack, executes while block if a > 0, repeats when the end of the loop is reached if the value at the top of the stack is greater than 0|

## Bit manipulation
To be implemented
