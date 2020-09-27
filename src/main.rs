mod error;
mod expr;
mod lexer;
mod parser;
use expr::Env;
use parser::parse;
use std::io::{self, Write};

fn interact(env: &mut Env) -> io::Result<()> {
    print!("math> ");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    match parse(&input) {
        Ok(exprs) => {
            for expr in exprs {
                println!("{}", expr.eval(env))
            }
        }
        Err(err) => println!("{}", err),
    }
    Ok(())
}

fn run_file(path: &str) -> Result<(), String> {
    let bytes = std::fs::read(path).map_err(|_| {
        format!(
            "I failed to read the input file :(\nIs the path below correct?\n{}\n",
            path
        )
    })?;
    let input = std::str::from_utf8(&bytes)
        .map_err(|_| "The input file doesn't seem to be valid utf-8 :(")?;
    let mut env = expr::Env::new();
    let exprs = parse(input)?;
    for expr in exprs {
        expr.eval(&mut env);
    }
    Ok(())
}

fn run() -> Result<(), String> {
    let args = std::env::args().collect::<Vec<String>>();
    match args.len() {
        // Run a file
        2 => run_file(args.get(1).unwrap())?,
        // Run in interactive mode
        1 => {
            let mut env = expr::Env::new();
            loop {
                interact(&mut env).map_err(|_| "An unexpected io error occured :(")?;
            }
        }
        // Fuck
        other => {
            return Err(format!(
                "I dont know what to do with {} arguments :(",
                other
            ))
        }
    };
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => (),
        Err(err) => println!("{}", err),
    }
}
