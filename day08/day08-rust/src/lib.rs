use std::convert::From;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Instruction {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl From<&str> for Instruction {
    fn from(line: &str) -> Self {
        let mut verse = line.trim_end().split(' ');
        let op = verse.next().unwrap();
        let arg = verse.next().unwrap().parse::<i32>().unwrap();
        match op {
            "acc" => Self::Acc(arg),
            "jmp" => Self::Jmp(arg),
            "nop" => Self::Nop(arg),
            _ => unimplemented!("{} is not supported", op),
        }
    }
}

impl Instruction {
    pub fn swap_jmp_nop(&mut self) {
        match *self {
            Self::Nop(arg) => *self = Self::Jmp(arg),
            Self::Jmp(arg) => *self = Self::Nop(arg),
            Self::Acc(_) => (),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Status {
    Normal(usize, Instruction),
    Terminated(i32),
    Repeated(i32),
}

#[derive(Debug, Eq, PartialEq)]
pub struct GamePad {
    ax: i32,
    ip: usize,
    codes: Vec<(Instruction, usize)>,
}

impl Clone for GamePad {
    fn clone(&self) -> Self {
        Self {
            ax: 0,
            ip: 0,
            codes: self.codes.clone(),
        }
    }
}

impl GamePad {
    pub fn new(input_file: &str) -> Self {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open(input_file).expect("open input file error");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("read input file error");
        Self {
            ax: 0,
            ip: 0,
            codes: buf.lines().map(|s| (Instruction::from(s), 0)).collect(),
        }
    }

    pub fn run(&mut self) -> Status {
        use Instruction::*;
        if self.ip >= self.codes.len() {
            return Status::Terminated(self.ax);
        }
        let (inst, run_times) = &mut self.codes[self.ip];
        if *run_times >= 1 {
            return Status::Repeated(self.ax);
        }
        let rip = self.ip;
        match inst {
            Acc(arg) => {
                self.ax += *arg;
                self.ip += 1;
            }
            Jmp(arg) => self.ip = (self.ip as i32 + *arg) as usize,
            Nop(_) => self.ip += 1,
        }
        *run_times += 1;
        Status::Normal(rip, *inst)
    }
}

pub fn part1_solution(pad: &mut GamePad) -> i32 {
    loop {
        match pad.run() {
            Status::Repeated(v) => return v,
            _ => (),
        }
    }
}

pub fn part2_solution(pad: &GamePad) -> Option<i32> {
    let mut pad_clone = pad.clone();
    let mut instructions = vec![];
    loop {
        if let Status::Normal(ip, inst) = pad_clone.run() {
            match inst {
                Instruction::Acc(_) => (),
                _ => instructions.push(ip),
            }
        } else {
            break;
        }
    }
    for ip in instructions {
        let mut pad_clone = pad.clone();
        pad_clone.codes[ip].0.swap_jmp_nop();
        loop {
            match pad_clone.run() {
                Status::Terminated(v) => return Some(v),
                Status::Repeated(_) => break,
                _ => (),
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let mut testcase = GamePad::new("../testcase1.txt");
        assert_eq!(part1_solution(&mut testcase), 5);
    }

    #[test]
    fn test_part2_solution() {
        let testcase = GamePad::new("../testcase1.txt");
        assert_eq!(part2_solution(&testcase), Some(8));
    }
}
