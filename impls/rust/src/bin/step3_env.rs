use mal::{
    env::{Env, EnvError},
    printer::pr_str,
    reader::{read_str, ReaderError},
    types::{IntOp, MalType},
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
enum ReplError {
    Fatal(String),
    Error(String),
}

impl std::fmt::Display for ReplError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fatal(s) | Self::Error(s) => {
                write!(f, "{}", s)
            }
        }
    }
}

impl From<String> for ReplError {
    fn from(s: String) -> ReplError {
        ReplError::Error(s)
    }
}

impl From<EnvError> for ReplError {
    fn from(e: EnvError) -> Self {
        ReplError::Error(format!("{e:?}"))
    }
}

// TODO: (S2 DEF) eval_ast on HashMap

fn eval_ast(ast: MalType, env: &mut Env) -> Result<MalType, ReplError> {
    // println!("eval_ast {:?}", ast);
    use MalType::*;
    Ok(match ast {
        Symbol(ref s) => match env.get(s) {
            Ok(val) => val,
            Err(_) => {
                return Err(ReplError::Error(format!("{s} not found")));
            }
        },
        List(maltypes) => List(
            maltypes
                .into_iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        Vector(xs) => Vector(
            xs.into_iter()
                .map(|x| eval(x, env))
                .collect::<Result<Vec<_>, _>>()?,
        ),
        _ => ast,
    })
}

fn eval(x: MalType, env: &mut Env) -> Result<MalType, ReplError> {
    // println!("eval {:?}", x);
    use MalType::*;
    match x {
        List(ref xs) if xs.is_empty() => Ok(x),
        List(ref xs) => {
            if let [Symbol(ref s), ref x, ref y] = &xs[..] {
                match s.as_str() {
                    "def!" => {
                        if let Symbol(ref key) = x {
                            // UGH clone (TODO)
                            let evaluated_value = eval(y.to_owned(), env)?;
                            env.set(key, evaluated_value.clone());
                            return Ok(evaluated_value);
                        } else {
                            return Err(ReplError::Error(format!("expected symbol after def!")));
                        }
                    }
                    "let*" => {
                        let mut inner_env = Env::new(Some(env));
                        if let List(ref binding_list) = x {
                            if binding_list.len() % 2 == 1 {
                                return Err(ReplError::Error(format!(
                                    "expected even length binding list, found {x:?}"
                                )));
                            };

                            let mut index = 0;
                            while let [ref symbol, ref bind_expr, ..] = &binding_list[index..] {
                                if let Symbol(ref s) = symbol {
                                    // pretty sure this will also clone
                                    let evaluated_bind_expr =
                                        eval(bind_expr.to_owned(), &mut inner_env)?;
                                    inner_env.set(s, evaluated_bind_expr)
                                } else {
                                    return Err(ReplError::Error(format!(
                                        "expected symbol in bindings list, got {symbol:?}"
                                    )));
                                }

                                println!("{inner_env:?}");
                                index += 2;
                            }

                            println!("{env:#?}");

                            return Ok(eval(y.to_owned(), &mut inner_env)?);
                        } else {
                            return Err(ReplError::Error(format!(
                                "expected binding list, found {x:?}"
                            )));
                        }
                    }
                    _ => {}
                }
            }
            match eval_ast(x, env)? {
                List(evaluated) if evaluated.len() == 3 => {
                    use IntOp::*;
                    match evaluated[..] {
                        [Op(ref op), Int(a), Int(b)] => match op {
                            Plus => Ok(Int(a + b)),
                            Minus => Ok(Int(a - b)),
                            Times => Ok(Int(a * b)),
                            Div => Ok(Int(a / b)),
                        },
                        _ => Err(ReplError::Error(format!(
                            "expected [ <Op>, <Int>, <Int> ], got {:?}",
                            evaluated
                        ))),
                    }
                }
                List(evaluated) => Err(ReplError::Error(format!(
                    "expected list of length 3, got {:?}",
                    evaluated
                ))),
                x => Err(ReplError::Error(format!(
                    "expected List of MalTypes, got {:?}",
                    x
                ))),
            }
        }
        _ => {
            // println!("x: {:?}", x);
            Ok(eval_ast(x, env)?)
        }
    }
}

fn print(x: MalType) -> String {
    pr_str(x, true)
}

fn rep(x: String, global_env: &mut Env) -> String {
    // println!("rep {}", x);
    match read(x) {
        Ok(t) => match eval(t, global_env) {
            Ok(maltype) => print(maltype),
            Err(s) => s.to_string(),
        },
        Err(ReaderError::_NoTokens) => "".to_string(),
        Err(e) => format!("{e}"),
    }
}

fn main() {
    let args = Args::parse();

    // create an env here
    let mut global_env = Env::new(None);
    global_env.set("+", MalType::Op(IntOp::Plus));
    global_env.set("-", MalType::Op(IntOp::Minus));
    global_env.set("*", MalType::Op(IntOp::Times));
    global_env.set("/", MalType::Op(IntOp::Div));

    // println!("{:?}", args);
    if args.test {
        match args.input {
            None => {
                println!("No input provided, exiting");
                exit(0)
            }
            Some(s) => {
                println!("input is {s}");
                println!("{}", rep(s, &mut global_env));
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
            exit(1);
        }

        match stdin().read_line(&mut input) {
            Ok(0) => exit(0),
            Ok(_) => {
                println!("{}", rep(input, &mut global_env))
            }
            Err(e) => {
                println!("Something bad happened: {e}");
                exit(1);
            }
        }
    }
}
