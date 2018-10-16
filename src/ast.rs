use lexer::*;
use token::Token;

#[derive(Debug, PartialEq)]
pub enum ASTKind {
    Int(i32),
    Ident(String),
    Add(Box<AST>, Box<AST>),
    // Minus(Box<AST>, Box<AST>),
}

#[derive(Debug, PartialEq)]
pub struct AST {
    pub kind: ASTKind,
}

pub struct Parser<'a> {
    tokens: &'a [Token],
    index: usize,
    pub result: Vec<AST>,
}

impl<'a> Parser<'a> {
    fn token_to_ast(t: Token) -> ASTKind {
        match t {
            Token::Int(i) => ASTKind::Int(i),
            Token::Ident(s) => ASTKind::Ident(s),
            _ => unimplemented!(),
        }
    }

    fn peek(&self) -> Option<Token> {
        if let Some(t) = self.tokens.get(self.index) {
            Some(t.clone())
        } else {
            None
        }
    }

    fn get(&mut self) -> Option<Token> {
        if let Some(t) = self.tokens.get(self.index) {
            self.index += 1;
            Some(t.clone())
        } else {
            None
        }
    }

    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            index: 0,
            result: vec![],
        }
    }

    fn primary(&mut self) -> AST {
        if let Some(token) = self.peek() {
            self.get();
            let kind = Parser::token_to_ast(token);
            AST { kind }
        } else {
            panic!("error!");
            AST {
                kind: ASTKind::Int(-114514),
            }
        }
    }

    fn additive(&mut self) -> AST {
        let mut left = self.primary();
        loop {
            let peeked = self.peek();
            if peeked != Some(Token::Plus) {
                break;
            }
            self.get();
            let right = self.primary();
            left = AST {
                kind: ASTKind::Add(Box::new(left), Box::new(right)),
            }
        }
        left
    }

    pub fn parse(&mut self) {
        let node = self.additive();
        self.result.push(node);
    }
}

mod tests {
    static tokens: [Token; 4] = [Token::Int(1), Token::Plus, Token::Int(2), Token::EOF];
    use super::*;
    #[test]
    fn parse_one_plus_two() {
        let mut p = Parser::new(&tokens);
        p.parse();
        assert_eq!(
            p.result,
            vec![AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Int(1)
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(2)
                    })
                )
            }]
        )
    }

    #[test]
    fn parse_one_plus_two_plus_three() {
        let t = vec![
            Token::Int(1),
            Token::Plus,
            Token::Int(2),
            Token::Plus,
            Token::Int(3),
            Token::EOF,
        ];
        let mut p = Parser::new(&t);
        p.parse();
        assert_eq!(
            p.result,
            vec![AST {
                kind: ASTKind::Add(
                    Box::new(AST {
                        kind: ASTKind::Add(
                            Box::new(AST {
                                kind: ASTKind::Int(1)
                            }),
                            Box::new(AST {
                                kind: ASTKind::Int(2)
                            })
                        )
                    }),
                    Box::new(AST {
                        kind: ASTKind::Int(3)
                    })
                )
            }]
        )
    }

    #[test]
    fn test_peek() {
        let mut p = Parser::new(&tokens);
        assert_eq!(p.peek(), Some(Token::Int(1)));
        assert_eq!(p.index, 0);
    }

    #[test]
    fn test_get() {
        let mut p = Parser::new(&tokens);
        assert_eq!(p.get(), Some(Token::Int(1)));
        assert_eq!(p.get(), Some(Token::Plus));
        assert_eq!(p.index, 2);
    }
}