use itertools::iproduct;
use std::collections::HashMap;

pub trait Coordinator {
    fn genarate_neighbors(&self) -> Vec<Box<Self>>;
    fn new_coordinate(coords: &[i32]) -> Box<Self>;
}

pub struct Coordinate3D([i32; 3]);

impl Coordinator for Coordinate3D {
    fn genarate_neighbors(&self) -> Vec<Box<Self>> {
        iproduct!((-1..2), (-1..2), (-1..2))
            .map(|(x, y, z)| Box::new(Self([self.0[0] + x, self.0[1] + y, self.0[2] + z])))
            .collect()
    }

    fn new_coordinate(coords: &[i32]) -> Box<Self> {
        if coords.len() != 3 {
            panic!("invalid axes number");
        }
        Self([coords[0], coords[1], coords[2]])
    }
}

pub struct Coordinate4D([i32; 4]);

impl Coordinator for Coordinate4D {
    fn genarate_neighbors(&self) -> Vec<Box<Self>> {
        iproduct!((-1..2), (-1..2), (-1..2), (-1..2))
            .map(|(x, y, z, w)| {
                Box::new(Self([
                    self.0[0] + x,
                    self.0[1] + y,
                    self.0[2] + z,
                    self.0[3] + w,
                ]))
            })
            .collect()
    }

    fn new_coordinate(coords: &[i32]) -> Box<Self> {
        if coords.len() != 4 {
            panic!("invalid axes number");
        }
        Self([coords[0], coords[1], coords[2], coords[3]])
    }
}

pub struct Dimensions<T: Coordinator> {
    cells: HashMap<T, bool>,
    dims: usize,
    axis_mins: Vec<i32>,
    axis_maxs: Vec<i32>,
}

impl<T> Dimensions<T: Coordinator> {
    pub fn new<T>(dims: usize, data: &str) -> Self {
        let mut cells = HashMap::new();
        for (y, line) in data.split('\n').enumerate() {
            for (x, c) in line.trim_end().as_bytes() {
                let coord = T::new(&[x, y, 0, 0]);
                if x == b'#' {}
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert!(true);
    }
}
