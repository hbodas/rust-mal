use mal::{
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

#[derive(Debug, Clone)]
struct ReplError(String);

impl std::fmt::Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ReplError: {}", self.0)
    }
}

impl From<String> for ReplError {
    fn from(s: String) -> ReplError {
        ReplError(s.clone())
    }
}

// TODO: (S2 DEF) eval_ast on HashMap

fn eval_ast(ast: MalType) -> Result<MalType, ReplError> {
    // println!("eval_ast {:?}", ast);
    use MalType::*;
    Ok(match ast {
        Symbol(s) => Op(s.parse()?),
        List(maltypes) => List(
            maltypes
                .into_iter()
                .map(|mtype: MalType| eval(mtype))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Vector(xs) => Vector(xs.into_iter().map(eval).collect::<Result<Vec<_>, _>>()?),
        _ => ast,
    })
}

fn eval(x: MalType) -> Result<MalType, ReplError> {
    // println!("eval {:?}", x);
    use MalType::*;
    match x {
        List(ref xs) if xs.is_empty() => Ok(x),
        List(_) => match eval_ast(x)? {
            List(evaluated) if evaluated.len() == 3 => match evaluated[..] {
                [Op(op), Int(a), Int(b)] => Ok(Int(op.exec(a, b))),
                _ => Err(ReplError(format!(
                    "expected [ <Op>, <Int>, <Int> ], got {:?}",
                    evaluated
                ))),
            },
            List(evaluated) => Err(ReplError(format!(
                "expected list of length 3, got {:?}",
                evaluated
            ))),
            x => Err(ReplError(format!("expected List of MalTypes, got {:?}", x))),
        },
        _ => {
            // println!("x: {:?}", x);
            Ok(eval_ast(x)?)
        }
    }
}

fn print(x: MalType) -> String {
    pr_str(x, true)
}

fn rep(x: String) -> String {
    // println!("rep {}", x);
    match read(x) {
        Ok(t) => match eval(t) {
            Ok(maltype) => print(maltype),
            Err(s) => s.to_string(),
        },
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
