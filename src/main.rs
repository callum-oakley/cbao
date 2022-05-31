use {
    anyhow::Result,
    std::{
        cmp::Ordering,
        env, fs,
        io::{self, prelude::*},
    },
};

mod reader;
mod value;

fn run(source: &str) -> Result<()> {
    for value in reader::Reader::new(source) {
        println!("{}", value?);
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    run(&fs::read_to_string(path)?)
}

fn prompt() -> Result<()> {
    print!("\n> ");
    io::stdout().flush()?;
    Ok(())
}

fn run_repl() -> Result<()> {
    prompt()?;
    for line in io::stdin().lock().lines() {
        if let Err(err) = run(&line?) {
            eprintln!("Error: {:?}", err);
        }
        prompt()?;
    }
    Ok(())
}

fn main() -> Result<()> {
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
