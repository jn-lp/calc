// The task of the work is to implement a lexical and syntactic analyzer of an arithmetic expression using any programming language.

// It is necessary that the analyzer checks for the following types of errors:
// 3. errors at the end of the expression (for example, the expression cannot end with any algebraic operation);
// 4. errors in the middle of the expression (double operations, lack of operations before or between parentheses, operations* or / after an open parenthesis, etc.); errors related to the use of parentheses (unequal number of open and closed parentheses, incorrect order of parentheses, empty parentheses).

mod token;
mod analyzer;
mod parser;

use std::io;
use crate::parser::Parser;

fn main() {
    // read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // parse input
    let mut parser = Parser::new(input);
    parser.parse();

    // print result
    println!("{:?}", parser.analyzer.stack);
    println!("{:?}", parser.analyzer.errors);
}
