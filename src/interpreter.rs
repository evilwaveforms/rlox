use crate::{
    environment::Environment,
    expr::{Assign, Binary, Call, Expr, Grouping, Logical, Unary, Variable},
    scanner::Literal,
    scanner::Token,
    scanner::TokenType,
    stmt::{Block, Expression, If, Print, Stmt, Var, While},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use std::{
    fmt,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Debug, PartialEq)]
pub enum Data {
    Number(f64),
    Str(String),
    Bool(bool),
    Func,
    Nil,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Number(num) => write!(f, "{}", num),
            Data::Str(str) => write!(f, "{}", str),
            Data::Bool(bool) => write!(f, "{}", bool),
            Data::Nil => write!(f, "nil"),
            Data::Func => write!(f, "func"),
        }
    }
}

impl LoxCallable for Data {}

trait LoxCallable {
    fn arity() {
        todo!();
    }
    fn call(&self, interpreter: &Interpreter, arguments: Vec<Data>) {
        todo!();
    }
}

#[derive(Debug, Clone)]
pub struct Interpreter {
    pub env: Rc<RefCell<Environment>>,
    pub repl: bool,
}

#[derive(Debug)]
pub enum Error {
    OperandNumberError(Token, String, String),
    OperandNumbersError(Token, String, String),
    AdditionError(Token, String, String),
    ValueError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::OperandNumberError(ref token, ref _left, ref right) => write!(
                f,
                "Operand must be a number. Token: {}, Right: {}",
                token, right
            ),
            Error::OperandNumbersError(ref token, ref left, ref right) => write!(
                f,
                "Operands must be numbers. Token: {}, Left: {}, Right: {}",
                token, left, right
            ),
            Error::AdditionError(ref token, ref left, ref right) => write!(
                f,
                "Operands must be two numbers or two strings. Token: {}, Left: {}, Right: {}",
                token, left, right
            ),
            Error::ValueError => write!(f, "error"),
        }
    }
}

impl Interpreter {
    pub fn call(&mut self, arguments: Vec<Stmt>) {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs_f32();
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) {
        for stmt in statements {
            self.execute(&stmt)
            // match execute(stmt) {
            //     Ok(value) => println!("{:?}", value.to_string()),
            //     Err(err) => {
            //         if !repl {
            //             panic!("Error: {:?}", err.to_string())
            //         }
            //         eprintln!("Error: {:?}", err.to_string())
            //     }
            // };
        }
    }

    pub fn evaluate(&mut self, expr: &Expr) -> Result<Data, Error> {
        match expr {
            Expr::Binary(expr) => self.evaluate_binary(&**expr),
            Expr::Grouping(grouping) => self.evaluate_grouping(&**grouping),
            Expr::Literal(lit) => self.evaluate_literal(lit.clone()),
            Expr::Unary(unary) => self.evaluate_unary(&**unary),
            Expr::Variable(expr) => self.evaluate_variable_expr(&**expr),
            Expr::Assign(expr) => self.evaluate_assign_expr(&**expr),
            Expr::Logical(expr) => self.evaluate_logical_expr(&**expr),
            Expr::Call(epxr) => todo!(),
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> () {
        match stmt {
            Stmt::Expression(expr) => self.evaluate_expression_stmt(&expr),
            Stmt::Print(expr) => self.evaluate_print_stmt(&expr),
            Stmt::Var(expr) => self.evaluate_var_stmt(&expr),
            Stmt::Block(stmt) => self.evaluate_block_stmt(&stmt),
            Stmt::If(stmt) => self.evaluate_if_stmt(&stmt),
            Stmt::While(stmt) => self.evaluate_while_stmt(&**stmt),
            Stmt::Function(stmt) => todo!(),
        }
    }

    fn execute_block(&mut self, statements: &Vec<Stmt>) {
        let curr_env = Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(self.env.clone()),
        }));

        let prev_env = self.env.clone();
        self.env = curr_env;

        for statement in statements {
            self.execute(&statement);
        }
        self.env = prev_env;
    }

    fn evaluate_block_stmt(&mut self, stmt: &Block) {
        self.execute_block(&stmt.statements)
    }

    fn evaluate_expression_stmt(&mut self, stmt: &Expression) {
        self.evaluate(&stmt.expression).unwrap();
    }

    fn evaluate_if_stmt(&mut self, stmt: &If) {
        if is_truthy(self.evaluate(&stmt.condition).unwrap()) {
            self.execute(&stmt.then_branch);
        } else if stmt.else_branch.is_some() {
            self.execute(&stmt.else_branch.clone().unwrap());
        }
    }

    fn evaluate_print_stmt(&mut self, stmt: &Print) {
        let val = self.evaluate(&stmt.expression);
        println!("{:?}", val);
    }

    fn evaluate_var_stmt(&mut self, stmt: &Var) {
        let mut value: Data = Data::Nil;
        if stmt.initializer.is_some() {
            value = self.evaluate(&stmt.initializer.clone().unwrap()).unwrap();
        }

        self.env
            .borrow_mut()
            .define(stmt.name.lexeme.clone(), value);
    }

    fn evaluate_while_stmt(&mut self, stmt: &While) {
        while is_truthy(self.evaluate(&stmt.condition).unwrap()) {
            self.execute(&stmt.body);
        }
        return;
    }

    fn evaluate_assign_expr(&mut self, expr: &Assign) -> Result<Data, Error> {
        let val = self.evaluate(&expr.value)?;
        self.env.borrow_mut().assign(&expr.name, &val);
        Ok(val)
    }

    fn evaluate_binary(&mut self, expr: &Binary) -> Result<Data, Error> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Plus => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left + right)),
                (Data::Str(left), Data::Str(right)) => Ok(Data::Str(left.to_owned() + &right)),
                _ => Err(Error::AdditionError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::Minus => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left - right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::Slash => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left / right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::Star => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Number(left * right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::Greater => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left > right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::GreaterEqual => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left >= right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::Less => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left < right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::LessEqual => match (&left, &right) {
                (Data::Number(left), Data::Number(right)) => Ok(Data::Bool(left <= right)),
                _ => Err(Error::OperandNumbersError(
                    expr.operator.clone(),
                    left.to_string(),
                    right.to_string(),
                )),
            },
            TokenType::BangEqual => Ok(Data::Bool(!is_equal(left, right))),
            TokenType::EqualEqual => Ok(Data::Bool(is_equal(left, right))),
            _ => Err(Error::ValueError),
        }
    }

    fn evaluate_call_expr(&mut self, expr: &Call) -> Result<Data, Error> {
        let callee = self.evaluate(&expr.callee)?;
        let mut arguments: Vec<Data> = Vec::new();

        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }
        // TODO: Prevent calling with strings.
        callee.call(self, arguments);
        todo!();
    }

    fn evaluate_literal(&mut self, literal: Literal) -> Result<Data, Error> {
        match literal {
            Literal::Str(str) => Ok(Data::Str(str)),
            Literal::Number(num) => Ok(Data::Number(num)),
            _ => Err(Error::ValueError),
        }
    }

    fn evaluate_logical_expr(&mut self, expr: &Logical) -> Result<Data, Error> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.ttype == TokenType::Or {
            if is_truthy(left.clone()) {
                return Ok(left);
            } else {
                if !is_truthy(left.clone()) {
                    return Ok(left);
                }
            }
        }
        Ok(self.evaluate(&expr.right))?
    }

    fn evaluate_grouping(&mut self, grouping: &Grouping) -> Result<Data, Error> {
        self.evaluate(&grouping.expression)
    }

    fn evaluate_unary(&mut self, expr: &Unary) -> Result<Data, Error> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.ttype {
            TokenType::Minus => match right {
                Data::Number(right) => Ok(Data::Number(-right)),
                _ => Err(Error::OperandNumberError(
                    expr.operator.clone(),
                    String::new(),
                    right.to_string(),
                )),
            },
            TokenType::Bang => Ok(Data::Bool(is_truthy(right))),
            _ => return Err(Error::ValueError),
        }
    }

    fn evaluate_variable_expr(&mut self, expr: &Variable) -> Result<Data, Error> {
        match self.env.borrow_mut().get(&expr.name) {
            Ok(var) => Ok(var),
            Err(_) => Err(Error::ValueError),
        }
    }
}
fn is_truthy(data: Data) -> bool {
    match data {
        Data::Nil => false,
        Data::Bool(val) => val,
        _ => true,
    }
}

fn is_equal(a: Data, b: Data) -> bool {
    if a == Data::Nil && b == Data::Nil {
        return true;
    }
    if a == Data::Nil {
        return false;
    }
    a == b
}
