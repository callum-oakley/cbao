mod error;
mod eval;
mod primitives;
mod reader;
mod value;

use {
    error::{ErrorData, Result},
    std::{
        fs,
        io::{self, prelude::*},
    },
    value::Env,
};

fn run_source(source: &str) -> Result<()> {
    let prelude = Env::prelude();
    for value in reader::read(source) {
        println!("{}", eval::eval(&value?, &prelude)?);
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    run_source(&fs::read_to_string(path)?)
}

fn prompt(s: &str) -> Result<()> {
    print!("{}", s);
    io::stdout().flush()?;
    Ok(())
}

fn run_repl() -> Result<()> {
    let mut input = String::new();
    prompt("\n> ")?;
    for line in io::stdin().lock().lines() {
        input += &line?;
        match run_source(&input) {
            Err(err) => {
                if let ErrorData::UnexpectedEof = err.data {
                    input.push('\n');
                    prompt("  ")?;
                    continue;
                }
                error::report(&err);
            }
            Ok(()) => (),
        };
        input.clear();
        prompt("\n> ")?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<_> = std::env::args().collect();
    match args.len() {
        1 => run_repl(),
        2 => run_file(&args[1]),
        // assumes -e <form> for now
        _ => run_source(&args[2]),
    }
}

fn main() {
    if let Err(err) = run() {
        error::report(&err);
        std::process::exit(1);
    }
}
