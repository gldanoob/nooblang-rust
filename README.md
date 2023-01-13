# NOOBLANG interpreter written in Rust (not blazingly fast)

**The Perfect (Esoteric) Programming Language for Beginners**
- ❌ No symbols (except `"`)
- ❌ No confusing `for`, `while` loops, functions
- ❌ No fancy data structures (`arrays` etc.)
- ❌ No variable scoping rules and stuff
- ✔️ Everything done with `if`s and procedures (`run`ning lines of code)

<br>

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

<br>

## Pascal's triangle
No arrays? Not a problem:
```
write "Enter number of rows (1 ~ 10): "
n be num read
run 25 to 26 if n mod 1 isnt 0 or n below 1 or n above 10
a be 1
i be 0
    s be ""
    j be i
    x be 0
    run 29 plus i
        run 20 to 23 if j is 0
        run j plus 38
        x be y
        run j plus 39
        x be x plus y
        run j plus 28
        s be s plus " " plus text x

        j be j minus 1
    run 10 to 19
    write s plus " 1"
    i be i plus 1
    end if i is n
run 6 to 23

write "Need integer between 1 and 10 inclusive"
run 1 to 23

a be x
b be x
c be x
d be x
e be x
f be x
g be x
h be x
k be x
l be x

y be a
y be b
y be c
y be d
y be e
y be f 
y be g
y be h
y be k
y be l
```
Only letters are allowed in variables names. Why? Because who would name their kid "Bobby123"?

<br><br>

# TODO
- Escape characters
- Prove turing completeness (Probably a brainf interpreter)
- Fix column numbers of errors
- Bytecode compiler? Nah, I don't have time


