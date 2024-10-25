use crate::enums::*;

pub struct Parser {
    tokens: Vec<Token>,
    expressions: Vec<Expr>,
    current: usize
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
            expressions: vec![]
        }
    }

    /// Parses the entire token stream.
    ///
    /// This is the main entry point for the parser.
    pub fn parse(&mut self) -> Vec<Expr> {
        while self.current < self.tokens.len() {
            let expr = self.parse_statement();
            self.expressions.push(expr);
        }

        return self.expressions.clone();
    }

    /// Parses any statement, whatsoever it may be, by calling the appropriate function.
    fn parse_statement(&mut self) -> Expr {
        let token = self.peek();
        match token {
            Token::Plus | Token::Minus | Token::Star | Token::Slash | Token::RParen | Token::LBracket | Token::Comma | Token::Semicolon => {
                self.parse_expression()
            },
            Token::Not | Token::LArrow | Token::RArrow | Token::Equality => {
                self.parse_comparison()
            }
            Token::Equal => {
                self.parse_assignment()
            },
            Token::Dot => {
                self.advance(); // Consume `.`
                self.parse_property_access()
            },
            Token::Let => {
                self.parse_declaration()
            },
            Token::Identifier(_name) => {
                self.parse_identifier()
            },
            Token::Func => {
                self.parse_function(false)
            },
            Token::If => {
                self.parse_conditional()
            },
            Token::While => {
                self.parse_while()
            },
            Token::For => {
                self.parse_for()
            },
            Token::Return => {
                self.parse_return()
            },
            Token::Number(_) => {
                self.parse_number()
            },
            Token::Import => {
                self.parse_import()
            },
            Token::Class => {
                self.parse_class()
            },
            Token::String(_) => {
                self.parse_string()
            }
            Token::LParen => {
                self.parse_function(true)
            },
            _ => todo!("Token: {:?}", self.peek())
        }
    }

    /// Parses a class declaration.
    fn parse_class(&mut self) -> Expr {
        self.advance(); // Consume `class`
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for class declaration, got {:?}", self.peek())
        };

        let body = self.parse_block();

        let result = Expr::DecClass(
            name,
            Box::new(body)
        );

        return result;
    }

    /// Parses a while loop.
    fn parse_while(&mut self) -> Expr {
        self.advance(); // Consume `while`
        let condition = self.parse_comparison();
        let body = self.parse_block();

        let result = Expr::While(
            Box::new(condition),
            Box::new(body)
        );

        return result;
    }

    /// Parses a for loop.
    fn parse_for(&mut self) -> Expr {
        self.advance(); // Consume `for`
        let iterator = match self.advance() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for for loop, got {:?}", self.peek())
        };
        self.advance(); // Consume `in`

        let iterable = self.parse_statement();
        let body = self.parse_block();

        let result = Expr::For(
            iterator,
            Box::new(iterable),
            Box::new(body)
        );

        return result;
    }

    /// Parses comparisons, such as `==`, `!=`, `>`, `<`, `>=`, `<=`.
    ///
    /// Can also parse a unary, like `1` for `true`.
    fn parse_comparison(&mut self) -> Expr {
        let left = match self.peek() {
            Token::String(value) => Expr::String(value),
            Token::Identifier(name) => Expr::Identifier(name),
            Token::Number(value) => Expr::Number(value),
            _ => panic!("Expected a number, string, or identifier, got {:?}", self.peek())
        };

        self.advance();
        match self.peek() {
            Token::Not => {
                self.advance(); // Consume `!`
                
                match self.peek() {
                    Token::Equal => {
                        self.advance(); // Consume `=`
                        let right = self.parse_expression();

                        let result = Expr::BinOp(
                            Operator::NotEqual,
                            Box::new(left),
                            Box::new(right)
                        );

                        return result;
                    },
                    _ => {
                        panic!("Expected `!=`, got {:?}", self.peek());
                    }
                }
            },
            Token::Equality => {
                self.advance(); // Consume `=`
                let right = self.parse_expression();

                let result = Expr::BinOp(
                    Operator::Equality,
                    Box::new(left),
                    Box::new(right)
                );

                return result;
            },
            Token::LArrow => { // <
                self.advance(); // Consume `<`

                match self.peek() {
                    Token::Equal => {
                        self.advance(); // Consume `=`
                        let right = self.parse_expression();

                        let result = Expr::BinOp(
                            Operator::LesserEqual,
                            Box::new(left),
                            Box::new(right)
                        );

                        return result;
                    },
                    _ => {}
                }

                let right = self.parse_expression();

                let result = Expr::BinOp(
                    Operator::Lesser,
                    Box::new(left),
                    Box::new(right)
                );

                return result;
            },
            Token::RArrow => { // >
                self.advance(); // Consume `>`

                match self.peek() {
                    Token::Equal => {
                        self.advance(); // Consume `=`
                        let right = self.parse_expression();

                        let result = Expr::BinOp(
                            Operator::GreaterEqual,
                            Box::new(left),
                            Box::new(right)
                        );

                        return result;
                    },
                    _ => {}
                }

                let right = self.parse_expression();

                let result = Expr::BinOp(
                    Operator::Greater,
                    Box::new(left),
                    Box::new(right)
                );

                return result;
            },
            _ => {
                // panic!("Expected condition, got {:?}", self.peek());
                return left;
            }
        }
    }

    /// Parses an if statement, including else if and else.
    fn parse_conditional(&mut self) -> Expr {
        self.advance(); // Consume `if`

        // let condition = self.parse_condition();
        let condition = self.parse_comparison();
        let body = self.parse_block();

        let mut else_body = vec![];
        match self.peek() {
            Token::Else => {
                self.advance(); // Consume `else`
                match self.peek() {
                    Token::If => {
                        else_body.push(self.parse_conditional());
                    },
                    _ => {
                        else_body = self.parse_block();
                    }
                }
            },
            _ => {}
        }

        let result = Expr::If(
            Box::new(condition),
            Box::new(body),
            Box::new(else_body)
        );

        return result;
    }

    /// Parses an import statement.
    fn parse_import(&mut self) -> Expr {
        self.advance(); // Consume `import`
        let name = match self.advance() {
            Token::String(name) => name,
            _ => panic!("Expected an identifier for import, got {:?}", self.peek())
        };

        let result = Expr::Import(name);

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    /// Parses a return statement.
    fn parse_return(&mut self) -> Expr {
        self.advance(); // Consume `return`
        let value = self.parse_expression();

        let result = Expr::Return(Box::new(value));

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    /// Parses a function call.
    fn parse_call(&mut self) -> Expr {
        let name = match self.advance() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for function call, got {:?}", self.peek())
        };

        self.advance(); // Consume `(`

        let mut args = vec![];
        loop {
            let token = self.peek();
            match token {
                Token::RParen => {
                    self.advance(); // Consume `)`
                    break;
                },
                Token::LParen => {
                    let arg = self.parse_function(true);
                    args.push(arg);
                },
                Token::Plus => {
                    break;
                }
                Token::Comma => {
                    self.advance(); // Consume `,`
                    continue;
                },
                Token::Semicolon => {
                    self.advance(); // Consume `;`
                    break;
                },
                Token::EOF => {
                    break;
                },
                _ => {
                    let arg = self.parse_expression();
                    args.push(arg);
                }
            }
        }

        let result = Expr::CallFunc(
            Box::new(Expr::Identifier(name)),
            Box::new(args)
        );

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    /// Parses JUST the arguments within parens.
    fn parse_args(&mut self) -> Vec<Expr> {
        self.advance(); // Consume `(`

        let mut args = vec![];
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
                Token::Plus => {
                    break;
                }
                Token::Comma => {
                    self.advance(); // Consume `,`
                    continue;
                },
                Token::Semicolon => {
                    self.advance(); // Consume `;`
                    break;
                },
                Token::EOF => {
                    break;
                },
                _ => {
                    let arg = self.parse_expression();
                    args.push(arg);
                }
            }
        }

        return args;
    }

    /// Parses a function declaration.
    fn parse_function(&mut self, is_anonymous: bool) -> Expr {
        let mut name = String::new();
        if !is_anonymous {
            self.advance(); // Consume `func`
            name = match { self.advance() } {
                Token::Identifier(name) => name,
                _ => panic!("Expected an identifier for function declaration, got {:?}", self.peek())
            };
        }
        else {
            name = "anonymous".to_string();
        }

        println!("Parsing function {:?}", name);

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
    /// Example:
    /// ```rs
    /// someThing.someProp; // This will consume the semicolon.
    /// ```
    fn parse_identifier(&mut self) -> Expr {
        let ident = match self.advance() {
            Token::Identifier(name) => {
                if name == "pass" {
                    match self.peek() {
                        Token::Semicolon => {
                            self.advance(); // Consume `;`
                        },
                        _ => {}
                    }
                    return Expr::Pass;
                }
                name
            },
            _ => panic!("Expected an identifier, got ?")
        };

        match self.peek() {
            Token::LParen => {
                self.retreat(); // `parse_call` requires the identifier to be the current token.
                return self.parse_call();
            }
            Token::LBracket => {
                self.retreat(); // `parse_array_access` requires the identifier to be the current token.
                return self.parse_array_access();
            }
            Token::Semicolon => {
                self.advance(); // Consume `;`
                return Expr::Identifier(ident);
            }
            Token::Plus | Token::Minus => {
                self.advance();
                match self.peek() {
                    Token::Equal => {
                        self.retreat();
                        self.retreat();
                        return self.parse_shorthand_assignment();
                    },
                    _ => {
                        self.retreat();
                    }
                }
            }
            Token::Dot => {
                self.retreat();
                return self.parse_property_access();
            }
            _ => {}
        }

        let result = Expr::Identifier(ident);

        return result;
    }

    fn parse_array_access(&mut self) -> Expr {
        let ident = match self.peek() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier, got {:?}", self.peek())
        };

        self.advance();
        self.advance();
        let index = self.parse_expression();
        self.advance();

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        let result = Expr::ArrayAccess(ident, Box::new(index));

        return result;
    }

    /// Parses a declaration, which is assumed to be a `let` statement.
    fn parse_declaration(&mut self) -> Expr {
        self.advance(); // Consume identifier

        let name = match self.peek() {
            Token::Identifier(name) => name,
            _ => panic!("Expected an identifier for declaration, got {:?}", self.peek())
        };

        self.advance();

        let typ = match self.peek() {
            Token::Colon => {
                self.advance();
                match self.advance() {
                    Token::Identifier(name) => {
                        match name.as_str() {
                            "Number" => MType::Number,
                            "String" => MType::String,
                            "Function" => MType::Function,
                            "Class" => MType::Class,
                            "Struct" => MType::Struct,
                            "Unknown" => MType::Undefined,
                            _ => {
                                panic!("Unknown type: {:?}", name);
                            }
                        }
                    },
                    _ => MType::Undefined
                }
            }
            _ => MType::Undefined
        };

        self.advance();

        let mut value = self.parse_statement();

        match self.peek() {
            Token::Plus => {
                self.advance();
                value = Expr::BinOp(Operator::Add, Box::new(value), Box::new(self.parse_expression()));
            },
            _ => {}
        }

        let result = Expr::BinOp(Operator::Declare(typ), Box::new(Expr::Identifier(name)), Box::new(value));

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    fn parse_assignment(&mut self) -> Expr {
        self.retreat();
        self.expressions.pop().unwrap();
        let assignee = match self.peek() {
            Token::Identifier(name) => Expr::Identifier(name),
            _ => panic!("Expected an identifier for assignment, got {:?}", self.peek())
        };
        self.advance();
        self.advance();

        let value = self.parse_expression();

        let result = Expr::BinOp(Operator::Assign, Box::new(assignee), Box::new(value));

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    fn parse_shorthand_assignment(&mut self) -> Expr {
        let assignee = match self.advance() {
            Token::Identifier(name) => Expr::Identifier(name),
            _ => panic!("Expected an identifier for assignment, got {:?}", self.peek())
        };

        let operator = match self.advance() {
            Token::Plus => Operator::Add,
            Token::Minus => Operator::Subtract,
            _ => panic!("Expected a shorthand operator, got {:?}", self.peek())
        };

        self.advance(); // Consume `=`

        let value = self.parse_expression();

        let addition = Expr::BinOp(operator, Box::new(assignee.clone()), Box::new(value));
        let result = Expr::BinOp(Operator::Assign, Box::new(assignee), Box::new(addition));

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }

        return result;
    }

    /// Parses an expression.
    ///
    /// Example:
    /// 1 + 2 + 3
    /// [1, 2, 3]
    /// "Hello"
    fn parse_expression(&mut self) -> Expr {
        let mut result = match self.peek() {
            Token::String(_) => {
                self.parse_string()
            },
            Token::Identifier(_) => {
                self.parse_identifier()
            },
            Token::Number(_) => {
                self.parse_number()
            },
            Token::LBracket => {
                self.parse_array()
            },
            Token::RBracket => {
                self.advance();
                return Expr::Array(Box::new(vec![]));
            },
            Token::Plus | Token::Minus => { // +=
                self.retreat();
                self.parse_shorthand_assignment()
            }
            _ => panic!("Expected a number, string, or identifier, got {:?}", self.peek())
        };

        loop {
            let operator = self.peek();
            let operator = match operator {
                Token::Plus => Operator::Add,
                Token::Minus => Operator::Subtract,
                Token::Star => Operator::Multiply,
                Token::Slash => Operator::Divide,
                Token::RParen => break,
                Token::Comma => break,
                Token::Semicolon => break,
                _ => {
                    break
                }
            };

            self.advance();

            let right = self.parse_statement();

            result = Expr::BinOp(operator, Box::new(result), Box::new(right));
        }

        match self.peek() {
            Token::Semicolon => {
                self.advance(); // Consume `;`
            },
            _ => {}
        }
        
        return result;
    }

    fn parse_array(&mut self) -> Expr {
        self.advance(); // Consume `[`
        let mut elements = vec![];
        loop {
            let token = self.peek();
            match token {
                Token::RBracket => {
                    self.advance(); // Consume `]`
                    break;
                },
                Token::LCurly => { // Likely the start of a for loop, and this was an inline array.
                    break;
                },
                Token::Comma => {
                    self.advance(); // Consume `,`
                    continue;
                },
                Token::Semicolon => {
                    self.advance(); // Consume `;`
                    break;
                },
                _ => {
                    let element = self.parse_expression();
                    elements.push(element);
                }
            }
        }

        let result = Expr::Array(Box::new(elements));

        return result;
    }

    /// Parses a `Token::Number` into an `Expr::Number`, or a Token::Identifier into an `Expr::Identifier`.
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
    fn parse_string(&mut self) -> Expr {
        let token = self.advance();
        let result = match token {
            Token::String(value) => Expr::String(value),
            _ => panic!("Expected a string, got {:?}", token)
        };

        return result;
    }

    /// Parses a property access.
    ///
    /// Assumes that the object we are accessing from is the last expression in `self.expressions`
    fn parse_property_access(&mut self) -> Expr {
        let object = match self.peek() {
            Token::Identifier(name) => Expr::Identifier(name),
            _ => panic!("Expected an identifier for property access, got {:?}", self.peek())
        };
        self.advance();
        self.advance();

        let property = self.parse_expression();

        let result = Expr::PropertyAccess(
            Box::new(object),
            Box::new(property)
        );

        return result;
    }

    /// Moves the current token pointer back by one.
    /// Minimize usage of this function.
    fn retreat(&mut self) {
        self.current -= 1;
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

