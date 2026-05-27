use crate::{
    ast::{
        Expression, Program,
        Statement::{self},
    },
    environment::Environment,
    object::Object,
};

pub fn eval_program(program: &Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in &program.statements {
        result = eval_statement(stmt, env);
        if let Object::Return(val) = result {
            return *val;
        }
    }
    result
}

fn eval_statement(stmt: &Statement, env: &mut Environment) -> Object {
    match stmt {
        Statement::ExpressionStmt(expr) => eval_expression(expr, env),
        Statement::Let { name, value } => {
            let val = eval_expression(value, env);
            env.set(name.clone(), val);
            Object::Null
        }
        Statement::Return(expr) => {
            let val = eval_expression(expr, env);
            Object::Return(Box::new(val))
        }
        _ => Object::Null,
    }
}

fn eval_expression(expr: &Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(n) => Object::Integer(*n),
        Expression::Identifier(name) => match env.get(name) {
            Some(v) => v,
            None => Object::Null,
        },
        Expression::BooleanLiteral(b) => Object::Boolean(*b),
        Expression::Prefix { operator, right } => {
            let right_obj = eval_expression(right, env);
            eval_prefix(operator, right_obj)
        }
        Expression::Infix {
            left,
            operator,
            right,
        } => {
            let left_obj = eval_expression(left, env);
            let right_obj = eval_expression(right, env);
            eval_infix(operator, left_obj, right_obj)
        }
        Expression::If {
            condition,
            consequence,
            alternative,
        } => eval_if(condition, consequence, alternative.as_deref(), env),
        _ => Object::Null,
    }
}

fn eval_prefix(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang(right),
        "-" => eval_minus_prefix(right),
        _ => Object::Null,
    }
}

fn eval_bang(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix(right: Object) -> Object {
    match right {
        Object::Integer(n) => Object::Integer(-n),
        _ => Object::Null,
    }
}

fn eval_infix(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => eval_integer_infix(operator, *l, *r),
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Null,
        },
        _ => Object::Null,
    }
}

fn eval_integer_infix(operator: &str, left: i64, right: i64) -> Object {
    match operator {
        "+" => Object::Integer(left + right),
        "-" => Object::Integer(left - right),
        "*" => Object::Integer(left * right),
        "/" => Object::Integer(left / right),
        "<" => Object::Boolean(left < right),
        ">" => Object::Boolean(left > right),
        "==" => Object::Boolean(left == right),
        "!=" => Object::Boolean(left != right),
        _ => Object::Null,
    }
}

fn eval_if(
    condition: &Expression,
    consequence: &Vec<Statement>,
    alternative: Option<&[Statement]>,
    env: &mut Environment,
) -> Object {
    let cond = eval_expression(condition, env);

    if is_truthy(cond) {
        eval_block(consequence, env)
    } else if let Some(alt) = alternative {
        eval_block(alt, env)
    } else {
        Object::Null
    }
}

fn eval_block(statements: &[Statement], env: &mut Environment) -> Object {
    let mut result = Object::Null;
    for stmt in statements {
        result = eval_statement(stmt, env);
        if let Object::Return(_) = &result {
            return result;
        }
    }
    result
}

fn is_truthy(obj: Object) -> bool {
    match obj {
        Object::Null => false,
        Object::Boolean(false) => false,
        _ => true,
    }
}
