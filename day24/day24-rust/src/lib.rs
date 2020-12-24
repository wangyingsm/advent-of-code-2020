use num::complex::Complex;
use std::collections::HashMap;

pub enum Orientation {
    West(Complex<i32>),
    SouthWest(Complex<i32>),
    SouthEast(Complex<i32>),
    East(Complex<i32>),
    NorthEast(Complex<i32>),
    NorthWest(Complex<i32>),
}

impl From<&[u8]> for Orientation {
    fn from(value: &[u8]) -> Self {
        match &value[..usize::min(value.len(), 2)] {
            [b'w', _] | [b'w'] => Self::O_WEST,
            [b's', b'w'] => Self::O_SOUTHWEST,
            [b's', b'e'] => Self::O_SOUTHEAST,
            [b'e', _] | [b'e'] => Self::O_EAST,
            [b'n', b'e'] => Self::O_NORTHEAST,
            [b'n', b'w'] => Self::O_NORTHWEST,
            _ => unimplemented!("no such orientation"),
        }
    }
}

impl Orientation {
    pub const ALL_ORIENTATIONS: &'static [Self] = &[
        Self::O_WEST,
        Self::O_SOUTHWEST,
        Self::O_SOUTHEAST,
        Self::O_EAST,
        Self::O_NORTHEAST,
        Self::O_NORTHWEST,
    ];
    const O_WEST: Self = Self::West(Complex::new(-2, 0));
    const O_SOUTHWEST: Self = Self::SouthWest(Complex::new(-1, 2));
    const O_SOUTHEAST: Self = Self::SouthEast(Complex::new(1, 2));
    const O_EAST: Self = Self::East(Complex::new(2, 0));
    const O_NORTHEAST: Self = Self::NorthEast(Complex::new(1, -2));
    const O_NORTHWEST: Self = Self::NorthWest(Complex::new(-1, -2));
}

fn parse_line(line: &str) -> Complex<i32> {
    use Orientation::*;
    let mut line = line.as_bytes();
    let mut ref_tile = Complex::new(0, 0);
    while !line.is_empty() {
        match Orientation::from(line) {
            West(v) | East(v) => {
                ref_tile += v;
                line = &line[1..];
            }
            SouthEast(v) | SouthWest(v) | NorthWest(v) | NorthEast(v) => {
                ref_tile += v;
                line = &line[2..];
            }
        }
    }
    ref_tile
}

pub fn read_input(input_file: &str) -> Vec<String> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .lines()
        .map(str::trim_end)
        .map(String::from)
        .collect()
}

pub fn init_tiles(lines: &[String]) -> HashMap<Complex<i32>, bool> {
    let mut tiles: HashMap<Complex<i32>, bool> = HashMap::new();
    for line in lines {
        let to_flip = parse_line(line);
        tiles
            .entry(to_flip)
            .and_modify(|e| *e = !*e)
            .or_insert(true);
    }
    tiles
}

pub fn count_all_black_tiles(tiles: HashMap<Complex<i32>, bool>) -> usize {
    tiles.iter().filter(|(_, v)| **v).count()
}

pub fn part1_solution(lines: &[String]) -> usize {
    let tiles = init_tiles(lines);
    count_all_black_tiles(tiles)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line("nwwswee"), Complex::new(0, 0));
    }

    #[test]
    fn test_part1() {
        let lines = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&lines), 10);
    }
}
