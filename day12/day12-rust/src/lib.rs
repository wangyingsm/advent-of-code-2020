use num::complex::Complex;
use std::convert::From;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Directions {
    East,
    South,
    West,
    North,
}

impl From<u8> for Directions {
    fn from(value: u8) -> Self {
        match value % 4 {
            0 => Directions::East,
            1 => Directions::South,
            2 => Directions::West,
            3 => Directions::North,
            _ => unreachable!("it can't happen"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Turn {
    Left,
    Right,
}

#[derive(Debug)]
pub struct Ferry1 {
    position: Complex<i32>,
    facing: Directions,
}

impl Ferry1 {
    pub fn new() -> Self {
        Self {
            position: Complex::new(0, 0),
            facing: Directions::East,
        }
    }

    pub fn move_direction(&mut self, d: Directions, value: i32) {
        match d {
            Directions::East => self.position += value,
            Directions::West => self.position -= value,
            Directions::North => self.position += Complex::new(0, value),
            Directions::South => self.position -= Complex::new(0, value),
        }
    }

    pub fn forward(&mut self, value: i32) {
        self.move_direction(self.facing, value);
    }

    pub fn turn(&mut self, t: Turn, value: i32) {
        let new_d = match t {
            Turn::Left => self.facing as u8 + 4 - ((value / 90) % 4) as u8,
            Turn::Right => self.facing as u8 + ((value / 90) % 4) as u8,
        };
        self.facing = Directions::from(new_d);
    }

    pub fn manhatten_distance(&self) -> i32 {
        self.position.re.abs() + self.position.im.abs()
    }

    pub fn do_instruction(&mut self, inst: char, value: i32) {
        match inst {
            'N' => self.move_direction(Directions::North, value),
            'S' => self.move_direction(Directions::South, value),
            'E' => self.move_direction(Directions::East, value),
            'W' => self.move_direction(Directions::West, value),
            'F' => self.forward(value),
            'L' => self.turn(Turn::Left, value),
            'R' => self.turn(Turn::Right, value),
            _ => unreachable!("invalid instruction"),
        }
    }
}

pub fn read_input(input_file: &str) -> Vec<(char, i32)> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.lines()
        .map(|s| {
            let b = s.trim_end().as_bytes();
            (
                b[0] as char,
                b[1..]
                    .iter()
                    .map(|&c| c as char)
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap(),
            )
        })
        .collect()
}

pub fn part1_solution(instructions: &[(char, i32)]) -> i32 {
    let mut ferry = Ferry1::new();
    for (inst, value) in instructions {
        ferry.do_instruction(*inst, *value);
    }
    ferry.manhatten_distance()
}

pub struct Ferry2 {
    position: Complex<i32>,
    waypoint: Complex<i32>,
}

impl Ferry2 {
    pub fn new() -> Self {
        Self {
            position: Complex::new(0, 0),
            waypoint: Complex::new(10, 1),
        }
    }

    pub fn move_waypoint(&mut self, d: Directions, value: i32) {
        match d {
            Directions::East => self.waypoint += value,
            Directions::West => self.waypoint -= value,
            Directions::North => self.waypoint += Complex::new(0, value),
            Directions::South => self.waypoint -= Complex::new(0, value),
        }
    }

    pub fn forward(&mut self, value: i32) {
        self.position += value * self.waypoint;
    }

    pub fn turn(&mut self, t: Turn, value: i32) {
        let comp_i = Complex::new(0, 1);
        let comp_neg_i = Complex::new(0, -1);
        for _ in 0..(value / 90 % 4) {
            self.waypoint *= match t {
                Turn::Left => comp_i,
                Turn::Right => comp_neg_i,
            }
        }
    }

    pub fn manhatten_distance(&self) -> i32 {
        self.position.re.abs() + self.position.im.abs()
    }

    pub fn do_instruction(&mut self, inst: char, value: i32) {
        match inst {
            'N' => self.move_waypoint(Directions::North, value),
            'S' => self.move_waypoint(Directions::South, value),
            'E' => self.move_waypoint(Directions::East, value),
            'W' => self.move_waypoint(Directions::West, value),
            'F' => self.forward(value),
            'L' => self.turn(Turn::Left, value),
            'R' => self.turn(Turn::Right, value),
            _ => unreachable!("invalid instruction"),
        }
    }
}

pub fn part2_solution(instructions: &[(char, i32)]) -> i32 {
    let mut ferry = Ferry2::new();
    for (inst, value) in instructions {
        ferry.do_instruction(*inst, *value);
    }
    ferry.manhatten_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&testcase), 25);
    }

    #[test]
    fn test_part2_solution() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(part2_solution(&testcase), 286);
    }
}
