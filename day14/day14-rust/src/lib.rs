use std::convert::From;
use std::ops::Add;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum MaskBit {
    Zero,
    One,
    X,
}

impl From<char> for MaskBit {
    fn from(bit: char) -> Self {
        match bit {
            '0' => Self::Zero,
            '1' => Self::One,
            'X' => Self::X,
            _ => unimplemented!("invalid bit set"),
        }
    }
}

impl MaskBit {
    pub fn part1_mask(&self, bit: u8) -> u8 {
        match self {
            Self::Zero => 0,
            Self::One => 1,
            Self::X => bit,
        }
    }

    pub fn part2_mask(&self, bit: u8) -> MaskBit {
        match self {
            Self::Zero => {
                if bit == 0 {
                    Self::Zero
                } else {
                    Self::One
                }
            }
            _ => *self,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Mask([MaskBit; 36]);

impl From<&str> for Mask {
    fn from(bits: &str) -> Self {
        if bits.len() != 36 {
            panic!("invalid mask length");
        }
        let mut mask = [MaskBit::Zero; 36];
        for (index, bit) in bits.chars().enumerate() {
            mask[index] = MaskBit::from(bit);
        }
        Self(mask)
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self([MaskBit::Zero; 36])
    }
}

impl Mask {
    pub fn part2_do_mask(&self, index: &Value) -> Vec<Key> {
        let result: Vec<_> = self
            .0
            .iter()
            .enumerate()
            .map(|(i, x)| x.part2_mask(index.0[i]))
            .collect();
        let x_len = result.iter().filter(|&&x| x == MaskBit::X).count();
        let mut keys = vec![];
        for i in 0..1 << x_len {
            let xs = format!("{:0width$b}", i, width = x_len);
            let mut j = 0;
            let key = result
                .iter()
                .map(|&x| match x {
                    MaskBit::One => 1u8,
                    MaskBit::Zero => 0u8,
                    MaskBit::X => {
                        let v = &xs[j..j + 1].parse::<u8>().unwrap();
                        j += 1;
                        *v
                    }
                })
                .fold(0, |x, y| (x << 1) + y as u64);
            keys.push(key);
        }
        keys
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Value([u8; 36]);

impl From<u64> for Value {
    fn from(v: u64) -> Self {
        let mut index = 35;
        let mut v = v;
        let mut values = [0; 36];
        while v > 0 {
            values[index] = (v % 2) as u8;
            v >>= 1;
            index -= 1;
        }
        Self(values)
    }
}

impl Value {
    pub fn part1_do_mask(&mut self, mask: &Mask) {
        for index in 0..36 {
            self.0[index] = mask.0[index].part1_mask(self.0[index]);
        }
    }

    pub fn value(&self) -> u64 {
        self.0.iter().fold(0, |x, y| (x << 1) + *y as u64)
    }
}

impl Add<u64> for &Value {
    type Output = u64;
    fn add(self, other: u64) -> Self::Output {
        self.value() + other
    }
}

use std::collections::HashMap;

type Key = u64;

#[derive(Debug, Eq, PartialEq)]
pub struct DockingProgram {
    memory: HashMap<Key, Value>,
    mask: Mask,
    is_part1: bool,
}

impl DockingProgram {
    pub fn new(is_part1: bool) -> Self {
        Self {
            memory: HashMap::new(),
            mask: Default::default(),
            is_part1,
        }
    }

    pub fn set_mask(&mut self, mask: &str) {
        self.mask = Mask::from(mask);
    }

    pub fn part1_set_memory(&mut self, index: Key, value: Value) {
        let mut value = value;
        value.part1_do_mask(&self.mask);
        self.memory.insert(index, value);
    }

    pub fn part2_set_memory(&mut self, index: Key, value: Value) {
        for index in self.mask.part2_do_mask(&Value::from(index)) {
            self.memory.insert(index, value);
        }
    }

    pub fn memory_sum(&self) -> u64 {
        self.memory.values().fold(0, |x, y| y + x)
    }

    pub fn run(&mut self, command: &str) {
        let mut parts = command.trim_end().split(" = ");
        let inst = parts.next().unwrap();
        let value = parts.next().unwrap();
        if inst == "mask" {
            self.set_mask(value);
        }
        if &inst[..3] == "mem" {
            let index = inst[4..inst.len() - 1].parse::<Key>().unwrap();
            if self.is_part1 {
                self.part1_set_memory(index, Value::from(value.parse::<u64>().unwrap()));
            } else {
                self.part2_set_memory(index, Value::from(value.parse::<u64>().unwrap()));
            }
        }
    }
}

pub fn read_input(input_file: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.lines().map(|s| s.to_string()).collect()
}

pub fn part1_solution(commands: &[String]) -> u64 {
    let mut program = DockingProgram::new(true);
    for command in commands.iter() {
        program.run(command);
    }
    program.memory_sum()
}

pub fn part2_solution(commands: &[String]) -> u64 {
    let mut program = DockingProgram::new(false);
    for command in commands.iter() {
        program.run(command);
    }
    program.memory_sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let commands = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&commands), 165);
    }

    #[test]
    fn test_find_all_indices() {
        let mask = Mask::from("000000000000000000000000000000X1001X");
        assert_eq!(mask.part2_do_mask(&Value::from(42)), vec![26, 27, 58, 59]);
        let mask = Mask::from("00000000000000000000000000000000X0XX");
        assert_eq!(
            mask.part2_do_mask(&Value::from(26)),
            vec![16, 17, 18, 19, 24, 25, 26, 27]
        );
    }

    #[test]
    fn test_part2() {
        let commands = read_input("../testcase2.txt");
        assert_eq!(part2_solution(&commands), 208);
    }
}
