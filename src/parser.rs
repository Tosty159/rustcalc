use crate::lexer::{Token, Lexer};

#[derive(Debug)]
pub enum ASTNode {
    Number(f64),
    BinaryOperator {
        lhs: Box<ASTNode>,
        op: char,
        rhs: Box<ASTNode>
    },
    UnaryOperator {
        op: char,
        operand: Box<ASTNode>,
    },
}

pub struct Parser<'a> {
    lexer: &'a mut Lexer<'a>,
    current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: &'a mut Lexer<'a>) -> Self {
        let current_token = lexer.next_token();
        Parser {
            lexer,
            current_token,
        }
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> ASTNode {
        let output_queue = self.shunting_yard();
        match self.rpn_to_ast(output_queue) {
            Some(node) => node,
            None => panic!("Malformed expression"),
        }
    }

    fn precedence(&mut self, tkn: Token) -> u16 {
        match tkn {
            Token::Operator(op) => {
                match op {
                    '+' | '-' => 1,
                    '*' | '/' => 2,
                    '^' => 3,
                    _ => 0,
                }
            },
            Token::UnaryOperator(_) => 10,
            _ => 0,
        }
    }

    fn is_left_associative(&mut self, tkn: Token) -> bool {
        match tkn {
            Token::Operator(op) => {
                match op {
                    '+' | '-' | '*' | '/' => true,
                    '^' => false,
                    _ => panic!("Invalid operator: {op}"),
                }
            },
            Token::UnaryOperator(_) => false,
            _ => panic!("Not an operator: {tkn:?}"),
        }
    }

    fn shunting_yard(&mut self) -> Vec<Token> {
        let mut output_queue: Vec<Token> = Vec::new();
        let mut operator_stack: Vec<Token> = Vec::new();

        while self.current_token != Token::EOF {
            match self.current_token {
                Token::Number(_) => output_queue.push(self.current_token),
                Token::Operator(_) => {
                    while let Some(&top_op) = operator_stack.last() {
                        if matches!(top_op, Token::Operator(_) | Token::UnaryOperator(_)) {
                            let op_precedence = self.precedence(self.current_token);
                            let top_precedence = self.precedence(top_op);

                            if top_precedence > op_precedence || (top_precedence == op_precedence && self.is_left_associative(top_op)) {
                                output_queue.push(operator_stack.pop().unwrap());
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    }
                    operator_stack.push(self.current_token);
                },
                Token::UnaryOperator(_) => operator_stack.push(self.current_token),
                Token::LParen => operator_stack.push(self.current_token),
                Token::RParen => {
                    while let Some(&op) = operator_stack.last() {
                        if op == Token::LParen {
                            break;
                        }

                        output_queue.push(operator_stack.pop().unwrap());
                    }
                    if !matches!(operator_stack.pop(), Some(Token::LParen)) {
                        panic!("Mismatched parentheses");
                    }
                },
                Token::EOF => break,
            }
            self.advance();
        }
        while let Some(&op) = operator_stack.last() {
            if op == Token::LParen {
                panic!("Mismatched parentheses");
            }
            output_queue.push(operator_stack.pop().unwrap());
        }

        output_queue
    }

    fn rpn_to_ast(&mut self, rpn_tokens: Vec<Token>) -> Option<ASTNode> {
        let mut stack: Vec<ASTNode> = Vec::new();

        for token in rpn_tokens {
            match token {
                Token::Number(num) => stack.push(ASTNode::Number(num)),
                Token::Operator(op) => {
                    if let (Some(right), Some(left)) = (stack.pop(), stack.pop()) {
                        stack.push(ASTNode::BinaryOperator {
                            lhs: Box::new(left),
                            op,
                            rhs: Box::new(right),
                        });
                    } else {
                        return None;
                    }
                },
                Token::UnaryOperator(op) => {
                    if let Some(opr) = stack.pop() {
                        stack.push(ASTNode::UnaryOperator {
                            op,
                            operand: Box::new(opr),
                        });
                    } else {
                        return None;
                    }
                }
                _ => {},
            }
        }

        if stack.len() == 1 {
            Some(stack.pop().unwrap())
        } else {
            None // Malformed RPN
        }
    }
}