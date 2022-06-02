mod error;
mod reader;
mod value;

use {
    error::Result,
    std::{
        cmp::Ordering,
        env,
        error::Error,
        fs,
        io::{self, prelude::*},
    },
};

fn run_source(source: &str) -> Result<()> {
    for value in reader::Reader::new(source) {
        println!("{}", value?);
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    run_source(&fs::read_to_string(path)?)
}

fn prompt() -> Result<()> {
    print!("\n> ");
    io::stdout().flush()?;
    Ok(())
}

fn run_repl() -> Result<()> {
    prompt()?;
    for line in io::stdin().lock().lines() {
        if let Err(err) = run_source(&line?) {
            eprintln!("Error: {:?}", err);
        }
        prompt()?;
    }
    Ok(())
}

fn run() -> Result<()> {
    let args: Vec<_> = env::args().collect();
    match args.len().cmp(&2) {
        Ordering::Less => run_repl(),
        Ordering::Equal => run_file(&args[1]),
        Ordering::Greater => {
            println!("Usage: {} [script]", args[0]);
            Ok(())
        }
    }
}

fn main() {
    if let Err(err) = run() {
        eprintln!("error: {}", err);
        let mut err: &(dyn Error) = &err;
        while let Some(e) = err.source() {
            err = e;
            eprintln!("caused by: {}", err);
        }
        std::process::exit(1);
    }
}
