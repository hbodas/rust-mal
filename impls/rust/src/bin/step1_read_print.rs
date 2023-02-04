use quux::{
    printer::pr_str,
    reader::{read_str, ReaderError},
    types::MalType,
};

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

fn read(x: String) -> Result<MalType, ReaderError> {
    read_str(x)
}

fn eval(x: MalType) -> MalType {
    x
}

fn print(x: MalType) -> String {
    pr_str(x)
}

fn rep(x: String) -> String {
    match read(x) {
        Ok(t) => print(eval(t)),
        Err(e) => format!("{e}"),
    }
}

fn main() {
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
