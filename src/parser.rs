use crate::analyzer::Analyzer;
use crate::token::Token;

#[derive(Debug)]
pub struct Parser {
    pub analyzer: Analyzer,
    input: String,
    position: usize,
}

impl Parser {
    pub fn new(input: String) -> Self {
        Self {
            analyzer: Analyzer::new(),
            input,
            position: 0,
        }
    }

    fn next_token(&mut self) -> Option<Token> {
        if self.position >= self.input.len() {
            return None;
        }

        let mut current_char = self.input.chars().nth(self.position).unwrap();
        self.position += 1;

        if current_char.is_ascii_digit() {
            let mut number = String::new();
            number.push(current_char);
            loop {
                if self.position >= self.input.len() {
                    break;
                }
                current_char = self.input.chars().nth(self.position).unwrap();
                if current_char.is_ascii_digit() {
                    number.push(current_char);
                    self.position += 1;
                } else {
                    break;
                }
            }
            if self.position < self.input.len() {
                current_char = self.input.chars().nth(self.position).unwrap();
                if current_char == '.' {
                    number.push(current_char);
                    self.position += 1;
                    loop {
                        if self.position >= self.input.len() {
                            break;
                        }
                        current_char = self.input.chars().nth(self.position).unwrap();
                        if current_char.is_ascii_digit() {
                            number.push(current_char);
                            self.position += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            return Some(Token::Number(number.parse().unwrap()));
        }

        if current_char.is_ascii_alphanumeric() {
            let mut variable = String::new();
            variable.push(current_char);
            loop {
                if self.position >= self.input.len() {
                    break;
                }
                current_char = self.input.chars().nth(self.position).unwrap();
                if current_char.is_ascii_alphanumeric() {
                    variable.push(current_char);
                    self.position += 1;
                } else {
                    break;
                }
            }
            return Some(Token::Variable(variable));
        }

        if current_char == '(' || current_char == ')' {
            return Some(Token::Parenthesis(current_char));
        }

        if current_char == '+' || current_char == '-' || current_char == '*' || current_char == '/'
        {
            return Some(Token::Operation(current_char));
        }

        None
    }

    pub fn parse(&mut self) {
        let mut current_token = self.next_token();
        while current_token.is_some() {
            self.analyzer.analyze(current_token.unwrap());
            current_token = self.next_token();
        }
    }
}
