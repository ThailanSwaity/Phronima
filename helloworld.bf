1 2 < if 
    1 65 write 
    1 read 91 < while 
        pop 

        1 read dup chout
        1 +
        1 swap write 

        1 read 91 <
    end
else 
    1 97 write
    1 read 123 < while
        pop 

        1 read dup chout 
        1 + 
        1 swap write 

        1 read 123 <
    end
end
10 chout
