mod error;
mod reader;
mod runtime;
mod value;

use {
    error::{Error, Result},
    std::{
        fs,
        io::{self, prelude::*},
    },
    value::Env,
};

fn run_source(source: &str) -> Result<()> {
    let prelude = Env::prelude();
    for value in reader::read(source) {
        println!("{}", runtime::eval(value?, &prelude)?);
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    run_source(&fs::read_to_string(path)?)
}

fn prompt() -> Result<()> {
    print!("> ");
    io::stdout().flush()?;
    Ok(())
}

fn run_repl() -> Result<()> {
    let mut input = String::new();
    println!();
    prompt()?;
    for line in io::stdin().lock().lines() {
        input += &line?;
        match run_source(&input) {
            Err(Error::UnexpectedEof) => {
                input.push('\n');
                prompt()?;
                continue;
            }
            Err(err) => {
                error::report(&err);
            }
            Ok(()) => (),
        };
        input.clear();
        println!();
        prompt()?;
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
