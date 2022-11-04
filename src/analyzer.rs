use crate::token::Token;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum AnalyzerError {
    ParseError(String),
}

impl std::fmt::Display for AnalyzerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AnalyzerError::ParseError(e) => write!(f, "{}", e),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum State {
    Start,
    Number,
    Variable,
    Parenthesis,
    Operation,
    Error(AnalyzerError),
}

#[derive(Debug)]
pub struct Analyzer {
    state: State,
    pub stack: Vec<Token>,
    pub errors: Vec<AnalyzerError>,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            state: State::Start,
            stack: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn analyze(&mut self, token: Token) {
        // 1. errors at the beginning of an arithmetic expression
        // (for example, the expression cannot begin with a closed parenthesis, algebraic operations * and /);
        match self.state {
            State::Start => match token {
                Token::Number(_) => self.state = State::Number,
                Token::Variable(_) => self.state = State::Variable,
                Token::Parenthesis('(') => self.state = State::Parenthesis,
                Token::Parenthesis(')') => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected parenthesis at the beginning of the expression".to_string(),
                    ))
                }
                Token::Operation(_) => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected operation at the beginning of the expression".to_string(),
                    ))
                }
                _ => {
                    self.state =
                        State::Error(AnalyzerError::ParseError("Unexpected token".to_string()))
                }
            },
            State::Number => match token {
                Token::Variable(_) => self.state = State::Variable,
                Token::Parenthesis(_) => self.state = State::Parenthesis,
                Token::Operation('+')
                | Token::Operation('-')
                | Token::Operation('*')
                | Token::Operation('/') => self.state = State::Operation,
                Token::Number(_) => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Error: unexpected number in the middle of the expression".to_string(),
                    ))
                }
                _ => {
                    self.state =
                        State::Error(AnalyzerError::ParseError("Unexpected token".to_string()))
                }
            },
            State::Variable => match token {
                Token::Number(_) => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected number after variable".to_string(),
                    ))
                }
                Token::Variable(_) => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected variable after variable".to_string(),
                    ))
                }
                Token::Parenthesis('(') => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected open parenthesis after variable".to_string(),
                    ))
                }
                Token::Operation('+')
                | Token::Operation('-')
                | Token::Operation('*')
                | Token::Operation('/') => self.state = State::Operation,
                _ => {
                    self.state =
                        State::Error(AnalyzerError::ParseError("Unexpected token".to_string()))
                }
            },
            State::Parenthesis => match token {
                Token::Number(_) => self.state = State::Number,
                Token::Variable(_) => self.state = State::Variable,
                Token::Parenthesis('(') => self.state = State::Parenthesis,
                _ => {
                    self.state =
                        State::Error(AnalyzerError::ParseError("Unexpected token".to_string()))
                }
            },
            State::Operation => match token {
                Token::Number(_) => self.state = State::Number,
                Token::Variable(_) => self.state = State::Variable,
                Token::Parenthesis('(') => self.state = State::Parenthesis,
                Token::Operation('+')
                | Token::Operation('-')
                | Token::Operation('*')
                | Token::Operation('/') => {
                    self.state = State::Error(AnalyzerError::ParseError(
                        "Unexpected operation after operation".to_string(),
                    ))
                }
                _ => {
                    self.state =
                        State::Error(AnalyzerError::ParseError("Unexpected token".to_string()))
                }
            },
            State::Error(_) => {}
        }

        println!("Token: {:?}", token);
        println!("State: {:?}", self.state);

        self.stack.push(token);
        if let State::Error(error) = &self.state {
            self.errors.push(error.clone());
            self.state = State::Start;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::token::Token;

    #[test]
    fn test_analyzer() {
        let mut analyzer = Analyzer::new();
        analyzer.analyze(Token::Number(1 as f64));
        analyzer.analyze(Token::Operation('+'));
        analyzer.analyze(Token::Number(2 as f64));
        analyzer.analyze(Token::Operation('-'));
        analyzer.analyze(Token::Number(3 as f64));
        analyzer.analyze(Token::Operation('*'));
        analyzer.analyze(Token::Number(4 as f64));
        analyzer.analyze(Token::Operation('/'));
        analyzer.analyze(Token::Number(5 as f64));
        analyzer.analyze(Token::Parenthesis('('));
        analyzer.analyze(Token::Number(6 as f64));
        analyzer.analyze(Token::Operation('+'));
        analyzer.analyze(Token::Number(7 as f64));
        analyzer.analyze(Token::Operation('-'));
        analyzer.analyze(Token::Number(8 as f64));
        analyzer.analyze(Token::Operation('*'));
        analyzer.analyze(Token::Number(9 as f64));
        analyzer.analyze(Token::Operation('/'));
        analyzer.analyze(Token::Number(10 as f64));
        analyzer.analyze(Token::Parenthesis(')'));

        assert_eq!(
            analyzer.stack,
            vec![
                Token::Number(1 as f64),
                Token::Operation('+'),
                Token::Number(2 as f64),
                Token::Operation('-'),
                Token::Number(3 as f64),
                Token::Operation('*'),
                Token::Number(4 as f64),
                Token::Operation('/'),
                Token::Number(5 as f64),
                Token::Parenthesis('('),
                Token::Number(6 as f64),
                Token::Operation('+'),
                Token::Number(7 as f64),
                Token::Operation('-'),
                Token::Number(8 as f64),
                Token::Operation('*'),
                Token::Number(9 as f64),
                Token::Operation('/'),
                Token::Number(10 as f64),
                Token::Parenthesis(')'),
            ]
        );
        assert_eq!(analyzer.errors.len(), 0);
    }

    #[test]
    fn test_analyzer_error() {
        let mut analyzer = Analyzer::new();
        analyzer.analyze(Token::Operation('+'));
        analyzer.analyze(Token::Variable(String::from('a')));

        assert_eq!(
            analyzer.errors,
            vec![
                AnalyzerError::ParseError("Unexpected operation at the beginning of the expression".to_string()),
            ]
        );
    }

    #[test]
    fn test_analyzer_error2() {
        let mut analyzer = Analyzer::new();
        analyzer.analyze(Token::Number(1 as f64));
        analyzer.analyze(Token::Number(2 as f64));

        assert_eq!(
            analyzer.errors,
            vec![
                AnalyzerError::ParseError("Error: unexpected number in the middle of the expression".to_string()),
            ]
        );
    }

    #[test]
    fn test_analyzer_error3() {
        let mut analyzer = Analyzer::new();
        analyzer.analyze(Token::Number(1 as f64));
        analyzer.analyze(Token::Operation('+'));
        analyzer.analyze(Token::Operation('-'));
        analyzer.analyze(Token::Number(3 as f64));

        assert_eq!(
            analyzer.errors,
            vec![
                AnalyzerError::ParseError("Unexpected operation after operation".to_string()),
            ]
        );
    }
}