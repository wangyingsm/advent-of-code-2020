use itertools::iproduct;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Index;

pub trait Coordinator {
    fn genarate_neighbors(&self) -> Vec<Box<Self>>;
    fn new_coordinate(coords: &[i32]) -> Box<Self>;
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate3D([i32; 3]);

impl Index<usize> for Coordinate3D {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= 3 {
            panic!("wrong axes number");
        }
        &self.0[idx]
    }
}

impl Coordinator for Coordinate3D {
    fn genarate_neighbors(&self) -> Vec<Box<Self>> {
        iproduct!((-1..2), (-1..2), (-1..2))
            .filter(|(x, y, z)| !(*x == 0 && *y == 0 && *z == 0))
            .map(|(x, y, z)| Box::new(Self([self.0[0] + x, self.0[1] + y, self.0[2] + z])))
            .collect()
    }

    fn new_coordinate(coords: &[i32]) -> Box<Self> {
        if coords.len() != 3 {
            panic!("invalid axes number");
        }
        Box::new(Self([coords[0], coords[1], coords[2]]))
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Coordinate4D([i32; 4]);

impl Index<usize> for Coordinate4D {
    type Output = i32;
    fn index(&self, idx: usize) -> &Self::Output {
        if idx >= 4 {
            panic!("wrong axes number");
        }
        &self.0[idx]
    }
}

impl Coordinator for Coordinate4D {
    fn genarate_neighbors(&self) -> Vec<Box<Self>> {
        iproduct!((-1..2), (-1..2), (-1..2), (-1..2))
            .filter(|(x, y, z, w)| !(*x == 0 && *y == 0 && *z == 0 && *w == 0))
            .map(|(x, y, z, w)| {
                Box::new(Self([
                    self.0[0] + x,
                    self.0[1] + y,
                    self.0[2] + z,
                    self.0[3] + w,
                ]))
            })
            .filter(|coord| coord.0.iter().any(|a| *a != 0))
            .collect()
    }

    fn new_coordinate(coords: &[i32]) -> Box<Self> {
        if coords.len() != 4 {
            panic!("invalid axes number");
        }
        Box::new(Self([coords[0], coords[1], coords[2], coords[3]]))
    }
}

pub struct Dimensions<T: Coordinator + Eq + Hash + Index<usize, Output = i32> + Copy + Debug> {
    cells: HashMap<T, bool>,
    dims: usize,
    axis_mins: Vec<i32>,
    axis_maxs: Vec<i32>,
}

impl<T> Dimensions<T>
where
    T: Coordinator + Eq + Hash + Index<usize, Output = i32> + Copy + Debug,
{
    pub fn new(dims: usize, data: &str) -> Self {
        let mut cells = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in data.split('\n').enumerate() {
            for (x, c) in line.trim_end().as_bytes().iter().enumerate() {
                let mut coord = [0].repeat(dims);
                coord[0] = x as i32;
                coord[1] = y as i32;
                let coord = T::new_coordinate(&coord);
                if *c == b'#' {
                    cells.insert(*coord, true);
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
        let mut axis_maxs = [0].repeat(dims);
        axis_maxs[0] = max_x as i32;
        axis_maxs[1] = max_y as i32;
        Self {
            cells,
            dims,
            axis_mins: [0].repeat(dims),
            axis_maxs,
        }
    }

    fn count_alive_neighbors(&self, coord: T) -> usize {
        let mut counter = 0;
        for neighbor in coord.genarate_neighbors() {
            if *self.cells.get(&*neighbor).unwrap_or(&false) {
                counter += 1
            }
        }
        counter
    }

    pub fn next_cycle(&mut self) {
        let mut new_cells = HashMap::new();
        let universe = match self.dims {
            3 => iproduct!(
                self.axis_mins[0] - 1..=self.axis_maxs[0] + 1,
                self.axis_mins[1] - 1..=self.axis_maxs[1] + 1,
                self.axis_mins[2] - 1..=self.axis_maxs[2] + 1
            )
            .map(|x| T::new_coordinate(&[x.0, x.1, x.2]))
            .collect::<Vec<_>>(),
            4 => iproduct!(
                self.axis_mins[0] - 1..=self.axis_maxs[0] + 1,
                self.axis_mins[1] - 1..=self.axis_maxs[1] + 1,
                self.axis_mins[2] - 1..=self.axis_maxs[2] + 1,
                self.axis_mins[3] - 1..=self.axis_maxs[3] + 1
            )
            .map(|x| T::new_coordinate(&[x.0, x.1, x.2, x.3]))
            .collect::<Vec<_>>(),
            _ => unimplemented!("only 3, 4 dims supported"),
        };
        for coord in universe {
            let coord = *coord;
            let neighbors = self.count_alive_neighbors(coord);
            let state = *self.cells.get(&coord).unwrap_or(&false);
            if state && (neighbors == 2 || neighbors == 3) {
                new_cells.insert(coord, true);
            }
            if !state && neighbors == 3 {
                new_cells.insert(coord, true);
                for i in 0..self.dims {
                    if coord[i] < self.axis_mins[i] {
                        self.axis_mins[i] = coord[i];
                    }
                    if coord[i] > self.axis_maxs[i] {
                        self.axis_maxs[i] = coord[i];
                    }
                }
            }
        }
        self.cells = new_cells;
    }

    pub fn count_all_alive_cells(&self) -> usize {
        self.cells.len()
    }
}

pub fn read_input(input_file: &str) -> String {
    std::fs::read_to_string(input_file).unwrap()
}

pub fn all_solutions<T>(init_matrix: &str, dims: usize, cycles: usize) -> usize
where
    T: Coordinator + Eq + Hash + Index<usize, Output = i32> + Copy + Debug,
{
    let mut universe = match dims {
        3 => Dimensions::<T>::new(3, init_matrix),
        4 => Dimensions::<T>::new(4, init_matrix),
        _ => panic!("only 3/4 dims supported"),
    };
    for _ in 0..cycles {
        universe.next_cycle();
    }
    universe.count_all_alive_cells()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(all_solutions::<Coordinate3D>(&testcase, 3, 6), 112);
    }

    #[test]
    fn test_part2() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(all_solutions::<Coordinate4D>(&testcase, 4, 6), 848);
    }
}
