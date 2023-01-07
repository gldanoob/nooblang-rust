use std::{
    fs::File,
    io::{self, BufReader},
    process,
};

mod errors;
mod lexer;
mod token;
mod ast;
mod parser;
fn main() -> io::Result<()> {
    let f = BufReader::new(File::open("test.txt")?);
    let stream = lexer::Reader::new(f).unwrap();

    let mut lex = lexer::Lexer::new(stream);

    let tokens = match lex.lex() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    for token in tokens {
        println!("{:?}", token);
    }
    Ok(())
}
