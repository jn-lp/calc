#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Variable(String),
    Parenthesis(char),
    Operation(char),
}