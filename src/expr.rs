// VALUE

#[derive(Debug, Copy, Clone)]
pub enum Value {
  Int(i64),
  Float(f64),
}

impl Value {
  fn f64(&self) -> f64 {
    match self {
      Value::Float(num) => *num,
      Value::Int(num) => *num as f64,
    }
  }
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Int(num) => write!(f, "{}", num),
      Float(num) => write!(f, "{}", num),
    }
  }
}

use Value::*;

// SCOPE
use std::collections::HashMap;

pub struct Env {
  vars: HashMap<String, Value>,
}

impl Env {
  pub fn new() -> Self {
    Env {
      vars: HashMap::new(),
    }
  }
}

// EXPRESSION

#[derive(Debug)]
pub enum Expr {
  Literal(Value),
  VarDeclaration(String, Box<Expr>),
  Var(String),
  Print(Box<Expr>),
  Multiplication(Box<Expr>, Box<Expr>),
  Division(Box<Expr>, Box<Expr>),
  Addition(Box<Expr>, Box<Expr>),
  Subtraction(Box<Expr>, Box<Expr>),
  Negative(Box<Expr>),
}

// EXPRESSION EVALUATION

fn eval_multiplication(left: &Expr, right: &Expr, env: &mut Env) -> Value {
  match (left.eval(env), right.eval(env)) {
    (Int(left), Int(right)) => Int(left * right),
    (left, right) => Float(left.f64() * right.f64()),
  }
}

fn eval_division(left: &Expr, right: &Expr, env: &mut Env) -> Value {
  match (left.eval(env), right.eval(env)) {
    (Int(left), Int(right)) => Int(left / right),
    (left, right) => Float(left.f64() / right.f64()),
  }
}

fn eval_addition(left: &Expr, right: &Expr, env: &mut Env) -> Value {
  match (left.eval(env), right.eval(env)) {
    (Int(left), Int(right)) => Int(left + right),
    (left, right) => Float(left.f64() + right.f64()),
  }
}

fn eval_subtraction(left: &Expr, right: &Expr, env: &mut Env) -> Value {
  match (left.eval(env), right.eval(env)) {
    (Int(left), Int(right)) => Int(left - right),
    (left, right) => Float(left.f64() - right.f64()),
  }
}

fn eval_negative(val: &Expr, env: &mut Env) -> Value {
  match val.eval(env) {
    Int(num) => Int(-num),
    Float(num) => Float(-num),
  }
}

fn eval_var_declaration(name: &str, expr: &Expr, env: &mut Env) -> Value {
  let val = expr.eval(env);
  env.vars.insert(name.to_owned(), val);
  val
}

fn eval_var(name: &str, env: &mut Env) -> Value {
  *env.vars.get(name).unwrap()
}

fn eval_print(val: &Expr, env: &mut Env) -> Value {
  let val = val.eval(env);
  println!("{}", val);
  val
}

impl Expr {
  pub fn eval(&self, env: &mut Env) -> Value {
    match self {
      Expr::Literal(val) => *val,
      Expr::VarDeclaration(name, expr) => eval_var_declaration(name, expr, env),
      Expr::Var(name) => eval_var(name, env),
      Expr::Print(val) => eval_print(val, env),
      Expr::Multiplication(left, right) => eval_multiplication(left, right, env),
      Expr::Division(left, right) => eval_division(left, right, env),
      Expr::Addition(left, right) => eval_addition(left, right, env),
      Expr::Subtraction(left, right) => eval_subtraction(left, right, env),
      Expr::Negative(val) => eval_negative(val, env),
    }
  }
}

// EXPRESSION CONSTRUCTORS

pub fn int(val: i64) -> Expr {
  Expr::Literal(Value::Int(val))
}
pub fn float(val: f64) -> Expr {
  Expr::Literal(Value::Float(val))
}
pub fn negative(val: Expr) -> Expr {
  Expr::Negative(Box::from(val))
}
pub fn add(left: Expr, right: Expr) -> Expr {
  Expr::Addition(Box::from(left), Box::from(right))
}
pub fn subtract(left: Expr, right: Expr) -> Expr {
  Expr::Subtraction(Box::from(left), Box::from(right))
}
pub fn multiply(left: Expr, right: Expr) -> Expr {
  Expr::Multiplication(Box::from(left), Box::from(right))
}
pub fn divide(left: Expr, right: Expr) -> Expr {
  Expr::Division(Box::from(left), Box::from(right))
}
