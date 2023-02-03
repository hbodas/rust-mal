use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

fn read(x: String) -> String {
    x
}

fn eval(x: String) -> String {
    x
}

fn print(x: String) -> String {
    x
}

fn rep(x: String) -> String {
    print(read(eval(x)))
}

fn main() {
    // TODO: Optional line interface
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
