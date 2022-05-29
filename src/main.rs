use {
    anyhow::Result,
    std::{
        env, fs,
        io::{self, prelude::*},
    },
};

mod reader;

fn run(source: &str) -> Result<()> {
    for token in reader::Tokens::new(source) {
        println!("{:?}", token?);
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
    if args.len() > 2 {
        println!("Usage: {} [script]", args[0]);
        Ok(())
    } else if args.len() == 2 {
        run_file(&args[1])
    } else {
        run_repl()
    }
}
