use std::{
    fs::File,
    io::{self, BufReader},
    process,
};

mod ast;
mod errors;
mod eval;
mod lexer;
mod parser;
mod token;

fn main() -> io::Result<()> {
    let f = BufReader::new(File::open(std::env::args().nth(1).unwrap())?);
    let mut stream = lexer::Reader::new(f).unwrap();

    let mut lex = lexer::Lexer::new(&mut stream);

    let tokens = match lex.lex() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    #[cfg(debug_assertions)]
    {
        for token in &tokens {
            println!("{:?}", token);
        }
        println!();
    }

    let mut parser = parser::Parser::new(tokens, &stream);
    let ast = match parser.parse() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    #[cfg(debug_assertions)]
    println!("{:#?}\n", ast);

    let mut eval = eval::Eval::new(&stream);
    let v = match eval.run_prog(&ast) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };
    #[cfg(debug_assertions)]
    {
        println!("--> {}", eval::Eval::display(&v));
        println!();
    }

    Ok(())
}
