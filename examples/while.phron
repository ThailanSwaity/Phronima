// This example is currently not supported for compilation.
// At the moment, I have not implemented "else" and "while" for 
// the brainfuck compiler

1 2 > if // if pops from the stack
    1 65 write 
    1 read 91 < while // while also pops from the stack

        1 read dup chout
        1 +
        1 swap write 

        1 read 91 < // The condition must be at the top of the stack at the end of the block
    end
else 
    1 97 write
    1 read 123 < while

        1 read dup chout 
        1 + 
        1 swap write 

        1 read 123 <
    end
end
10 chout
