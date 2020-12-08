use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub struct InputLine<'a> {
    pub least: usize,
    pub most: usize,
    pub letter: u8,
    pub code: &'a str,
}

impl<'a> From<&'a str> for InputLine<'a> {
    fn from(line: &'a str) -> Self {
        let mut fields = line.trim_end().split(' ');
        let r = fields.next().expect("wrong format input line");
        let letter = fields
            .next()
            .expect("no character for search")
            .trim_end_matches(':')
            .as_bytes()[0];
        let code = fields.next().expect("no code input");
        let mut fields = r.split('-');
        let least = fields
            .next()
            .expect("no at-least field input")
            .parse::<usize>()
            .expect("at-least field not a integer");
        let most = fields
            .next()
            .expect("no at-most field input")
            .parse::<usize>()
            .expect("at-most field not a integer");
        Self {
            least,
            most,
            letter,
            code,
        }
    }
}

impl<'a> InputLine<'a> {
    pub fn part1_valid(&self) -> bool {
        let count = self
            .code
            .as_bytes()
            .iter()
            .filter(|&c| c == &self.letter)
            .count();
        self.least <= count && count <= self.most
    }

    pub fn part2_valid(&self) -> bool {
        let least_valid = (self.code.as_bytes()[self.least - 1] == self.letter) as u8;
        let most_valid = (self.code.as_bytes()[self.most - 1] == self.letter) as u8;
        (least_valid ^ most_valid) == 1
    }
}

pub fn read_input(input_file: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("invalid input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.lines().map(|s| s.to_string()).collect()
}

pub fn part1_solution(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|&&s| {
            let l = InputLine::from(s);
            l.part1_valid()
        })
        .count()
}

pub fn part2_solution(lines: &[&str]) -> usize {
    lines
        .iter()
        .filter(|&&s| {
            let l = InputLine::from(s);
            l.part2_valid()
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::From;

    #[test]
    fn test_input_line_from() {
        let l = InputLine::from("7-8 x: xxnszxbxgxwxx\n");
        assert_eq!(
            l,
            InputLine {
                least: 7,
                most: 8,
                letter: b'x',
                code: "xxnszxbxgxwxx"
            }
        );
    }

    #[test]
    fn test_valid_func() {
        let l = InputLine::from("7-8 x: xxnszxbxgxwxx\n");
        assert!(l.part1_valid());
        assert!(l.part2_valid());
        let l = InputLine::from("1-10 j: jjjjjjjjjjjj\n");
        assert!(!l.part1_valid());
        assert!(!l.part2_valid());
    }
}
