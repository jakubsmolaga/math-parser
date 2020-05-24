use crate::error::print_err;
use crate::expr::{add, divide, float, int, multiply, negative, subtract, Expr};
use crate::lexer::{tokenize, Token};

type WrappedToken<'a> = (Token<'a>, usize);
type Tokens<'a> = [WrappedToken<'a>];
type ParseError<'a> = (WrappedToken<'a>, &'a str);
type ParseResult<'a> = Result<(&'a Tokens<'a>, Expr), ParseError<'a>>;

fn first<'a>(tokens: &'a Tokens) -> WrappedToken<'a> {
  tokens[0]
}

fn eat_one<'a>(tokens: &'a Tokens) -> (&'a Tokens<'a>, WrappedToken<'a>) {
  (&tokens[1..], tokens[0])
}

fn skip_one<'a>(tokens: &'a Tokens) -> &'a Tokens<'a> {
  &tokens[1..]
}

fn parse_additive<'a>(tokens: &'a Tokens) -> ParseResult<'a> {
  let (mut tokens, mut expr) = parse_multiplicative(&tokens)?;
  loop {
    match first(&tokens) {
      (Token::Plus, _) => {
        let (rest, _) = eat_one(&tokens);
        let (rest, other) = parse_multiplicative(&rest)?;
        expr = add(expr, other);
        tokens = rest;
      }
      (Token::Minus, _) => {
        let (rest, _) = eat_one(&tokens);
        let (rest, other) = parse_multiplicative(&rest)?;
        expr = subtract(expr, other);
        tokens = rest;
      }
      _ => return Ok((&tokens, expr)),
    };
  }
}

fn parse_multiplicative<'a>(tokens: &'a Tokens) -> ParseResult<'a> {
  let (mut tokens, mut expr) = parse_primary(&tokens)?;
  loop {
    match first(&tokens) {
      (Token::Star, _) => {
        let (rest, _) = eat_one(&tokens);
        let (rest, other) = parse_primary(&rest)?;
        expr = multiply(expr, other);
        tokens = rest;
      }
      (Token::Slash, _) => {
        let (rest, _) = eat_one(&tokens);
        let (rest, other) = parse_primary(&rest)?;
        expr = divide(expr, other);
        tokens = rest;
      }
      _ => return Ok((&tokens, expr)),
    };
  }
}

fn parse_primary<'a>(tokens: &'a Tokens) -> ParseResult<'a> {
  let (tokens, token) = eat_one(&tokens);
  match token {
    (Token::LeftParen, _) => {
      let (tokens, expr) = parse_additive(&tokens)?;
      if first(tokens).0 != Token::RightParen {
        return Err((first(tokens), "Hey, I expected a closing parenthesis here"));
      }
      let tokens = skip_one(&tokens);
      Ok((&tokens, expr))
    }
    (Token::Int(num), _) => Ok((&tokens, int(num))),
    (Token::Float(num), _) => Ok((&tokens, float(num))),
    (Token::Minus, _) => {
      let (tokens, expr) = parse_primary(&tokens)?;
      Ok((&tokens, negative(expr)))
    }
    (Token::LetKeyword, _) => match first(tokens) {
      (Token::Name(name), _) => {
        let tokens = skip_one(tokens);
        if first(tokens).0 != Token::Equals {
          return Err((first(tokens), "Hey, I expected \"=\" right here"));
        }
        let tokens = skip_one(tokens);
        let (tokens, expr) = parse_additive(tokens)?;
        Ok((
          tokens,
          Expr::VarDeclaration(name.to_owned(), Box::from(expr)),
        ))
      }
      token => Err((token, "Hey, I expected a name of a variable right here")),
    },
    (Token::PrintKeyword, _) => {
      let (tokens, expr) = parse_additive(tokens)?;
      Ok((tokens, Expr::Print(Box::from(expr))))
    }
    (Token::Name(name), _) => Ok((tokens, Expr::Var(name.to_owned()))),
    (Token::EOF, _) => Err((token, "Hey, I didn't expect the input to end right here")),
    token => Err((token, "Hey, I didn't expect this thing right here")),
  }
}

pub fn parse(input: &str) -> Result<Vec<Expr>, String> {
  let mut tokens = &tokenize(&input)?[..];
  let mut expressions = Vec::new();
  while first(tokens).0 != Token::EOF {
    let (unparsed, expr) =
      parse_additive(&tokens).map_err(|err| print_err(input, (err.0).1, err.1))?;
    expressions.push(expr);
    tokens = unparsed;
  }
  Ok(expressions)
}
