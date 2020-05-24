#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token<'a> {
  Int(i64),
  Float(f64),
  Name(&'a str),
  LetKeyword,
  PrintKeyword,
  Plus,
  Minus,
  Star,
  Slash,
  LeftParen,
  RightParen,
  Equals,
  EOF,
}

fn first(input: &str) -> char {
  input.chars().next().unwrap()
}

fn skip_char(input: &str) -> &str {
  &input[1..]
}

fn eat_while(input: &str, cond: fn(&char) -> bool) -> (&str, &str) {
  let pos = input.chars().take_while(cond).count();
  (&input[pos..], &input[..pos])
}

fn skip_whitespace(input: &str) -> &str {
  eat_while(&input, |c| c.is_ascii_whitespace()).0
}

fn eat_digits(input: &str) -> (&str, &str) {
  eat_while(&input, |c| c.is_ascii_digit())
}

fn eat_number(input: &str) -> Result<(&str, Token), Err> {
  let (rest, digits) = eat_digits(&input);
  if rest.is_empty() || first(rest) != '.' {
    let int = digits
      .parse::<i64>()
      .map_err(|_| failed_to_parse_number(&input))?;
    Ok((&rest, Token::Int(int)))
  } else {
    let rest = skip_char(&rest);
    let (rest, _) = eat_digits(&rest);
    let len = input.len() - rest.len();
    let float = input[..len]
      .parse::<f64>()
      .map_err(|_| failed_to_parse_number(&input))?;
    Ok((&rest, Token::Float(float)))
  }
}

fn eat_word(input: &str) -> Result<(&str, Token), Err> {
  let (rest, word) = eat_while(input, |c| c.is_ascii_alphabetic());
  let token = match word {
    "let" => Token::LetKeyword,
    "print" => Token::PrintKeyword,
    name => Token::Name(name),
  };
  Ok((rest, token))
}

fn eat_token(input: &str) -> Result<(&str, Token), Err> {
  let token = match first(input) {
    '+' => Token::Plus,
    '-' => Token::Minus,
    '*' => Token::Star,
    '/' => Token::Slash,
    '(' => Token::LeftParen,
    ')' => Token::RightParen,
    '=' => Token::Equals,
    c if c.is_ascii_alphabetic() => return eat_word(input),
    c if c.is_ascii_digit() => return eat_number(input),
    _ => return Err(unexpected_char(input)),
  };
  Ok((skip_char(input), token))
}

pub type Tokens<'a> = Vec<(Token<'a>, usize)>;

pub fn tokenize(input: &str) -> Result<Tokens, String> {
  let mut result: Vec<(Token, usize)> = Vec::new();
  let mut unprocessed = skip_whitespace(&input);
  if unprocessed.is_empty() {
    return Err("Didn't find any input. Give me something to parse next time!".to_owned());
  }
  while !unprocessed.is_empty() {
    let (rest, token) = eat_token(&unprocessed).map_err(|err| err.print(&input))?;
    result.push((token, input.len() - unprocessed.len()));
    unprocessed = skip_whitespace(&rest);
  }
  result.push((Token::EOF, input.trim_end().len()));
  Ok(result)
}

// -- Errors

fn unexpected_char(remaining_input: &str) -> Err {
  Err::new(
    remaining_input,
    "Sorry, I dont know what to do with this character :(".to_owned(),
  )
}

fn failed_to_parse_number(remaining_input: &str) -> Err {
  Err::new(
    remaining_input,
    "Sorry, I couldn't construct this number :( Make sure that it's not too big!".to_owned(),
  )
}

struct Err {
  msg: String,
  rest_len: usize,
}

use crate::error::print_err;
impl Err {
  fn new(remaining_input: &str, msg: String) -> Self {
    Err {
      rest_len: remaining_input.len(),
      msg,
    }
  }
  fn print(&self, original_input: &str) -> String {
    let pos = original_input.len() - self.rest_len;
    print_err(&original_input, pos, &self.msg)
  }
}
