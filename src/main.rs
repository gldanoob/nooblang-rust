use std::{fs::File, io::BufReader};

use errors::Errors;

mod ast;
mod errors;
mod eval;
mod lexer;
mod parser;
mod token;

fn main() {
    match run_file() {
        Ok(_) => (),
        Err(e) => {
            eprintln!("{}", e);
        }
    }
}

fn run_file() -> Result<(), Errors> {
    let filename = std::env::args().nth(1).ok_or(Errors::ArgumentError)?;
    let f = BufReader::new(File::open(filename).map_err(|_| Errors::IOError)?);
    let mut stream = lexer::Reader::new(f)?;

    let mut lex = lexer::Lexer::new(&mut stream);

    let tokens = lex.lex()?;

    #[cfg(debug_assertions)]
    {
        for token in &tokens {
            println!("{:?}", token);
        }
        println!();
    }

    let mut parser = parser::Parser::new(&tokens, &stream);
    let ast = parser.parse()?;

    #[cfg(debug_assertions)]
    println!("{:#?}\n", ast);

    let mut eval = eval::Eval::new(&ast, &stream);
    let v = eval.run_prog()?;

    #[cfg(debug_assertions)]
    {
        println!("--> {}", eval::Eval::display(&v));
        println!();
    }

    Ok(())
}
