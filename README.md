# Phronima
Phronima is a stack-based higher-level language for brainf*ck

# Intro 
What is brainf*ck? [Brainf*ck](https://en.wikipedia.org/wiki/Brainfuck) is an esoteric programming language modelled after the [Turing Machine](https://en.wikipedia.org/wiki/Turing_machine)
# Goals 
- [x] [Hello, world!](./examples/hellowrold.bf)
- [x] ([if, end](./examples/if.bf)) Control flow operators (if, else, while)
- [ ] Character input, number input, string input
- [ ] Verbose compiler errors
- [ ] Comments
- [ ] Strings
- [ ] Optimize compiled brainf*ck
- [ ] Turing completeness

# Modes 
There are two modes/subcommands
  'com': will compile the source code into brainf*ck code
  'sim': will simulate the program created by the source code 

NOTE: some operations do not currently work in 'compile' mode. Still a work in progress.

      cargo run -sim ./helloworld.bf

# Memory
Phronima uses an array of 256 bytes as its "memory", with the remaining 29,744 cells of brainf*ck to be used as a stack

There is no type system implemented and I only plan on having two: byte and pointer.
Be careful when writing with this language as it is very easy to cause stack underflow

# Supported operations
| operation|compiler   |simulator  |
|------------|-----------|-----------|
| push  |:heavy_check_mark: | :heavy_check_mark: |
| pop   |:heavy_check_mark: | :heavy_check_mark:|
| plus  |:heavy_check_mark: |:heavy_check_mark: |
| chout |:heavy_check_mark: |:heavy_check_mark: |
| numout| |:heavy_check_mark: |
| write |:heavy_check_mark: |:heavy_check_mark: |
| read  |:heavy_check_mark: |:heavy_check_mark: |
| mem   |:heavy_check_mark: |:heavy_check_mark: |
| if    |:heavy_check_mark: |:heavy_check_mark: |
| end   |:heavy_check_mark: |:heavy_check_mark: |
| else  | |:heavy_check_mark: |
| while | |:heavy_check_mark: |
| < (less than)    | |:heavy_check_mark: |
| > (greater than)    | |:heavy_check_mark: |
| = (equal to)    | |:heavy_check_mark: |
| swap  | |:heavy_check_mark: |
| dup   | |:heavy_check_mark: |

# Operation descriptions
  todo!
