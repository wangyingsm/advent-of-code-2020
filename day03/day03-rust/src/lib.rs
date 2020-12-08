use std::fs::File;
use std::io::Read;
use std::convert::From;

pub enum MapCell {
    Blank,
    Tree,
}

impl From<&u8> for MapCell {
    fn from(repr: &u8) -> Self {
        match repr {
            &b'.' => Self::Blank,
            &b'#' => Self::Tree,
            _ => unreachable!("should be either a blank `.` or a tree `#`"),
        }
    }
}

pub fn read_input(input_file: &str) -> Vec<Vec<MapCell>> {
    let mut file = File::open(input_file).expect("open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read input file");
    buf.lines().map(|s| s.as_bytes().iter().map(MapCell::from).collect::<Vec<_>>()).collect()
}

fn solution(tree_map: &[&[MapCell]], slope: (usize, usize)) -> u64 {
    let (rows, cols) = (tree_map.len(), tree_map[0].len());
    let mut c = 0;
    let mut tree_count = 0;
    for r in (0..rows).step_by(slope.1) {
        c = c % cols;
        match tree_map[r][c] {
            MapCell::Tree => tree_count += 1,
            _ => ()
        }
        c += slope.0;
    }
    tree_count
}

pub fn part1_solution(tree_map: &[&[MapCell]]) -> u64 {
    solution(tree_map, (3, 1))
}

pub fn part2_solution(tree_map: &[&[MapCell]]) -> u64 {
    let slopes = &[(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    slopes.iter().map(|&s| solution(tree_map, s)).product()
}
