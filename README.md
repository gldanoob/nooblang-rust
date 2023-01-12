# NOOBLANG interpreter written in Rust (not blazingly fast)

## The Perfect (Esoteric) Programming Language for Beginners
- (Almost) no symbols
- ~~No confusing `for`, `while` loops, functions~~
- ~~No fancy data structures (`arrays` etc.)~~
- ~~No variable scoping rules and stuff~~
- Everything done with `if`s and procedures (`run`ning lines of code)

# Examples
## To print prime numbers between 0 and 100:
```
x be 2                         
prime be yes                  
y be 1                     
y be y plus 1
run 8 to 11 if y is x      
prime be no if x mod y is 0
run 4 to 7
write x if prime 
x be x plus 1
end if x atleast 100
run 2 to 11
```

Most of the syntax shouldn't be hard to understand (they're designed for beginners),
but in case you're wondering, `run` runs the line(s) of code with the given line number (or range)

So `run 8 to 11 if y is x` runs lines 8-11 if `x` is equal to `y`  

## Implementing fixed-size arrays:
```
y be 10                 note        y = 10
index be 1              note        index = 1
run index plus 7        note        arr[index] = y
run index plus 13       note        x = arr[index]
write x                 note        print(x)    # 10

a be y
b be y
c be y
d be y
e be y

x be a
x be b
x be c
x be d
x be e
```
Only letters are allowed in variables names. Why? Because who would name their kid "Bobby123"?


# TODO
- Fix stack overflow (using another stack)
- Prove turing completeness (Probably a brainf interpreter)
- Fix column numbers of errors
- Bytecode compiler? Nah, I don't have time


