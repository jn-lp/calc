mod lib;

use std::io;
use crate::lib::parser::Parser;

fn get_args() -> std::env::Args {
    let mut args = std::env::args();
    args.next();
    args
}

fn main() {
    for arg in get_args() {
        match arg.as_str() {
            "-v" | "--version" => {
                println!("Version: {}", env!("CARGO_PKG_VERSION"));
                return;
            }
            "-h" | "--help" => {
                println!("Usage: calc [expression]");
                println!("Options:");
                println!("  -v, --version  Show version");
                println!("  -h, --help     Show this help");
                return;
            }
            _ => (),
        }
    }

    let mut expression_opt = None;
    for arg in get_args() {
        if expression_opt.is_none() {
            expression_opt = Some(arg);
        } else {
            println!("Error: too many arguments");
            return;
        }
    }

    let expression = match expression_opt {
        Some(expression) => expression,
        None => {
            println!("Enter an expression:");
            let mut expression = String::new();
            io::stdin().read_line(&mut expression).unwrap();
            expression
        }
    };

    let mut parser = Parser::new(expression);
    parser.parse();

    println!("Stack: {:?}", parser.analyzer.stack);
    println!("Errors: {:?}", parser.analyzer.errors);
}
