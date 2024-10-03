use crate::enums::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
    debug_mode: bool
}

impl Parser {
    pub fn new(tokens: Vec<Token>, debug_mode: bool) -> Self {
        Self {
            tokens,
            current: 0,
            debug_mode
        }
    }

    pub fn parse(&mut self) -> Vec<Expr> {
        let mut expressions = vec![];
        while self.current < self.tokens.len() {
            expressions.push(self.parse_statement())
        }

        return expressions;
    }

    fn parse_statement(&mut self) -> Expr {
        let token = self.peek();
        match token {
            Token::Let => {
                self.parse_declaration()
            },
            Token::Identifier(_name) => {
                self.parse_identifier()
            },
            Token::Func => {
                self.parse_function()
            }
            _ => todo!("Token: {:?}", self.peek())
        }
    }

    /// Parses a function declaration.
    ///
    /// Does consume semicolons.
    fn parse_function(&mut self) -> Expr {
        self.advance(); // Consume `func`
        let name = match { self.advance() } {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for function declaration, got {:?}", self.peek())
        };
        self.advance(); // Consume `(`

        let params = self.parse_params();

        let body = self.parse_block();

        let result = Expr::DecFunc(
            name,
            Box::new(params),
            Box::new(body)
        );
        
        return result;
    }

    /// Parses a block of code, which is assumed to be a series of statements.
    ///
    /// Does consume semicolons.
    fn parse_block(&mut self) -> Vec<Expr> {
        let mut block = vec![];
        loop {
            let token = self.peek();
            match token {
                Token::LCurly => {
                    self.advance(); // Consume `{`
                },
                Token::RCurly => {
                    self.advance(); // Consume `}`
                    break;
                },
                _ => {
                    let statement = self.parse_statement();
                    block.push(statement);
                }
            }
        }

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return block;
    }

    /// Parses the arguments of a function declaration.
    ///
    /// Use this to parse params during declaration, not during a call.
    ///
    /// Does consume semicolons.
    fn parse_params(&mut self) -> Vec<String> {
        let mut params = vec![];
        loop {
            let token = self.peek();
            match token {
                Token::RParen => {
                    self.advance(); // Consume `)`
                    break;
                },
                Token::LParen => {
                    self.advance(); // Consume `(`
                    continue;
                },
                Token::Comma => {
                    self.advance(); // Consume `,`
                    continue;
                },
                Token::Identifier(param) => {
                    self.advance(); // Consume identifier
                    params.push(param);
                },
                _ => {
                    panic!("Unexpected token while parsing params: {:?}", token);
                }
            }
        }

        return params;
    }

    /// Turns `Token::Identifier` into `Expr::Identifier`
    ///
    /// Does NOT consume semicolons.
    fn parse_identifier(&mut self) -> Expr {
        let ident = match self.advance() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier, got ?")
        };

        let result = Expr::Identifier(ident);

        return result;
    }

    /// Parses a declaration, which is assumed to be a `let` statement.
    ///
    /// Does consume semicolons.
    fn parse_declaration(&mut self) -> Expr {
        self.advance(); // Consume `let`
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for declaration, got {:?}", self.peek())
        };
        self.advance(); // Consume `=`
        
        let value = match self.peek() {
            Token::String(_) => {
                self.parse_string()
            },
            _ => self.parse_expression()
        };

        let result = Expr::BinOp(Operator::Declare, Box::new(Expr::String(name)), Box::new(value));

        // `parse_expression` consumes the semicolon, but `parse_string` does not.
        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    /// Parses an expression, which is assumed to be series of numbers and operators.
    ///
    /// Example: `1 + 2 + 3`
    ///
    /// Does consume semicolons.
    fn parse_expression(&mut self) -> Expr {
        let mut result = self.parse_number();

        loop {
            let operator = self.advance();
            let operator = match operator {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::Semicolon => {
                    break
                },
                _ => panic!("Expected an operator, got {:?}", operator)
            };

            let right = self.parse_number();

            result = Expr::BinOp(operator, Box::new(result), Box::new(right));
        }

        return result;
    }

    /// Parses a `Token::Number` into an `Expr::Number`, or a Token::Identifier into an `Expr::Identifier`.
    ///
    /// Does NOT consume semicolons.
    fn parse_number(&mut self) -> Expr {
        let token = self.advance();
        let result = match token {
            Token::Number(value) => Expr::Number(value),
            Token::Identifier(name) => Expr::Identifier(name),
            _ => panic!("Expected a number, got {:?}", token)
        };

        return result;
    }

    /// Parses a `Token::String` into an `Expr::String`.
    ///
    /// Does NOT consume semicolons.
    fn parse_string(&mut self) -> Expr {
        let token = self.advance();
        let result = match token {
            Token::String(value) => Expr::String(value),
            _ => panic!("Expected a string, got {:?}", token)
        };

        return result;
    }

    /// Returns the current token, then advances to the next one.
    fn advance(&mut self) -> Token {
        self.current += 1;

        match self.tokens.get(self.current - 1) {
            Some(token) => {
                return token.clone();
            },
            None => {
                Token::EOF
            }
        }

    }

    /// Returns the current token without advancing.
    fn peek(&self) -> Token {
        match self.tokens.get(self.current) {
            Some(token) => {
                return token.clone();
            },
            None => {
                Token::EOF
            }
        }
    }
}

