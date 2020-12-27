use std::unimplemented;

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
    Digits(i64, usize),
    Operators(u8),
    Parentheses(u8, usize),
}

pub enum CaculateMode {
    Add,
    Multiply,
}

fn next_token(data: &str) -> Token {
    let mut buf = vec![];
    let mut advance = 0;
    for (i, c) in data.chars().enumerate() {
        let i = i + 1;
        match c {
            '(' => return Token::Parentheses(c as u8, i),
            ')' => {
                if buf.is_empty() {
                    return Token::Parentheses(c as u8, i);
                } else {
                    return Token::Digits(
                        buf.iter().collect::<String>().parse::<i64>().unwrap(),
                        i - 1,
                    );
                }
            }
            '+' | '*' => return Token::Operators(c as u8),
            ' ' if !buf.is_empty() => {
                return Token::Digits(buf.iter().collect::<String>().parse::<i64>().unwrap(), i)
            }
            '0'..='9' => buf.push(c),
            _ => (),
        }
        advance = i;
    }
    Token::Digits(
        buf.iter().collect::<String>().parse::<i64>().unwrap(),
        advance,
    )
}

pub fn part1_do_math(data: &str) -> (i64, usize) {
    let mut result = 0;
    let data = data.trim_end();
    let mut advance = 0;
    let mut mode = CaculateMode::Add;
    while advance < data.trim_end().len() {
        let token = next_token(&data[advance..]);
        match token {
            Token::Parentheses(b')', a) => {
                advance += a;
                return (result, advance);
            }
            Token::Parentheses(b'(', a) => {
                advance += a;
                let (r, a) = part1_do_math(&data[advance..]);
                match mode {
                    CaculateMode::Add => result += r,
                    CaculateMode::Multiply => result *= r,
                }
                advance += a;
            }
            Token::Digits(o, a) => {
                match mode {
                    CaculateMode::Add => result += o,
                    CaculateMode::Multiply => result *= o,
                }
                advance += a;
            }
            Token::Operators(o) => {
                match o {
                    b'+' => mode = CaculateMode::Add,
                    b'*' => mode = CaculateMode::Multiply,
                    _ => unimplemented!("{} operation not supported.", o as char),
                }
                advance += 1;
            }
            t => unimplemented!("{:?} token not supported.", t),
        }
    }
    (result, advance)
}

pub fn read_input(input_file: &str) -> Vec<String> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(str::to_string)
        .collect()
}

pub fn all_solutions<T>(lines: &[String], f: &mut T) -> i64
where
    T: FnMut(&str) -> (i64, usize),
{
    lines.iter().map(|s| f(&s[..]).0).sum::<i64>()
}

pub fn part2_do_math(data: &str) -> (i64, usize) {
    let mut stack = vec![];
    let data = data.trim_end();
    let mut advance = 0;
    let mut mode = CaculateMode::Add;
    while advance < data.trim_end().len() {
        let token = next_token(&data[advance..]);
        match token {
            Token::Parentheses(b')', a) => {
                advance += a;
                return (stack.iter().product(), advance);
            }
            Token::Parentheses(b'(', a) => {
                advance += a;
                let (r, a) = part2_do_math(&data[advance..]);
                match mode {
                    CaculateMode::Add => {
                        let x = if !stack.is_empty() {
                            stack.pop().unwrap()
                        } else {
                            0
                        };
                        stack.push(r + x);
                    }
                    CaculateMode::Multiply => stack.push(r),
                }
                advance += a;
            }
            Token::Digits(o, a) => {
                match mode {
                    CaculateMode::Add => {
                        let x = if !stack.is_empty() {
                            stack.pop().unwrap()
                        } else {
                            0
                        };
                        stack.push(o + x);
                    }
                    CaculateMode::Multiply => stack.push(o),
                }
                advance += a;
            }
            Token::Operators(o) => {
                match o {
                    b'+' => mode = CaculateMode::Add,
                    b'*' => mode = CaculateMode::Multiply,
                    _ => unimplemented!("{} operation not supported.", o as char),
                }
                advance += 1;
            }
            t => unimplemented!("{:?} token not supported.", t),
        }
    }
    (stack.iter().product(), advance)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_token() {
        let mut advance = 0;
        let line = "1 + (2 * 3) + (4 * (5 + 6))";
        let ntoken = next_token(&line[advance..]);
        assert_eq!(ntoken, Token::Digits(1, 2));
        if let Token::Digits(_, a) = ntoken {
            advance += a;
            let ntoken = next_token(&line[advance..]);
            assert_eq!(ntoken, Token::Operators(b'+'));
        }
        if let Token::Operators(_) = ntoken {
            advance += 1;
            let ntoken = next_token(&line[advance..]);
            assert_eq!(ntoken, Token::Parentheses(b'(', 1));
        }
    }

    #[test]
    fn test_part1_do_math() {
        assert_eq!(part1_do_math("1 + 2 * 3 + 4 * 5 + 6").0, 71);
        assert_eq!(part1_do_math("1 + (2 * 3) + (4 * (5 + 6))").0, 51);
        assert_eq!(part1_do_math("2 * 3 + (4 * 5)").0, 26);
        assert_eq!(part1_do_math("5 + (8 * 3 + 9 + 3 * 4 * 3)").0, 437);
        assert_eq!(
            part1_do_math("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").0,
            12240
        );
        assert_eq!(
            part1_do_math("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").0,
            13632
        );
    }

    #[test]
    fn test_part2_do_math() {
        assert_eq!(part2_do_math("1 + 2 * 3 + 4 * 5 + 6").0, 231);
        assert_eq!(part2_do_math("1 + (2 * 3) + (4 * (5 + 6))").0, 51);
        assert_eq!(part2_do_math("2 * 3 + (4 * 5)").0, 46);
        assert_eq!(part2_do_math("5 + (8 * 3 + 9 + 3 * 4 * 3)").0, 1445);
        assert_eq!(
            part2_do_math("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))").0,
            669060
        );
        assert_eq!(
            part2_do_math("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2").0,
            23340
        );
    }
}
