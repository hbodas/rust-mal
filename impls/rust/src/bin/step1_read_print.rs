use quux::{
    printer::pr_str,
    reader::{read_str, ReaderError},
    types::MalType,
};

use clap::Parser;
use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

#[derive(Debug, Parser)]
struct Args {
    #[arg(long, required = false)]
    test: bool,

    #[arg(long)]
    input: Option<String>,
}

fn read(x: String) -> Result<MalType, ReaderError> {
    // println!("read {}", x);
    read_str(x)
}

fn eval(x: MalType) -> MalType {
    x
}

fn print(x: MalType) -> String {
    pr_str(x, true)
}

fn rep(x: String) -> String {
    // println!("rep {}", x);
    match read(x) {
        Ok(t) => print(eval(t)),
        Err(ReaderError::_NoTokens) => "".to_string(),
        Err(e) => format!("{e}"),
    }
}

fn main() {
    let args = Args::parse();
    // println!("{:?}", args);
    if args.test {
        match args.input {
            None => {
                println!("No input provided, exiting");
                exit(0)
            }
            Some(s) => {
                println!("input is {s}");
                println!("{}", rep(s));
                return;
            }
        }
    };

    // TODO: add an option to just test something
    // TODO: stuff to move around lines
    loop {
        let mut input = String::new();

        print!("user> ");
        if let Err(e) = stdout().flush() {
            println!("Something bad happened: {e}");
            exit(1)
        }

        match stdin().read_line(&mut input) {
            Ok(0) => exit(0),
            Ok(_) => {
                println!("{}", rep(input))
            }
            Err(e) => {
                println!("Something bad happened: {e}");
                exit(1);
            }
        }
    }
}
