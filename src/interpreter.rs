use crate::parser::ASTNode;

pub fn interpret(ast: ASTNode) -> f64 {
    match ast {
        ASTNode::Number(num) => num,
        ASTNode::UnaryOperator { operand, op } => {
            let opr = interpret(*operand);
            match op {
                '+' => opr,
                '-' => -opr,
                _ => panic!("Invalid unary operator: {op}"),
            }
        },
        ASTNode::BinaryOperator { lhs, op, rhs } => {
            let left = interpret(*lhs);
            let right = interpret(*rhs);

            match op {
                '+' => left + right,
                '-' => left - right,
                '*' => left * right,
                '/' => left / right,
                '^' => left.powf(right),
                _ => panic!("Invalid binary oeprator: {op}"),
            }
        }
    }
}