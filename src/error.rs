pub fn print_err(input: &str, pos: usize, msg: &str) -> String {
  let line_num = input[..pos].lines().count();
  let line_end = pos + input[pos..].find('\n').unwrap_or(input.len() - pos);
  let line_start = pos
    - input[..pos]
      .chars()
      .rev()
      .position(|c| c == '\n')
      .unwrap_or(pos);
  let line = &input[line_start..line_end];
  let spaces: String = std::iter::repeat(' ')
    .take(pos - line_start + line_num.to_string().len() + 2)
    .collect();
  format!("{}\n{}| {}\n{}^", msg, line_num, line, spaces)
}
