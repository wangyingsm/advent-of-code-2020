use ndarray::{concatenate, s, Array1, Array2, Axis};
#[macro_use]
extern crate lazy_static;

peg::parser!(grammar parse_tile() for str {
    pub rule parse_tile_id() -> usize
        = "Tile " id:$(['0'..='9']+) ":" { id.parse().unwrap() }

    pub rule parse_border() -> (u32, u32)
        = line:$(['#' | '.']+) {
            let line = line.chars().map(|x| match x {
                '#' => '1',
                '.' => '0',
                _ => unimplemented!("invalid image pixel"),
            }).collect::<String>();
            (u32::from_str_radix(&line, 2).unwrap(),
            u32::from_str_radix(&line.chars().rev().collect::<String>(), 2).unwrap())
        }

    pub rule parse_sub_image() -> Array1<u8>
        = line:$(['#' | '.']+) {
            let mut arr = unsafe { Array1::<u8>::uninitialized(line.len()) };
            for (i, c) in line.chars().enumerate() {
                match c {
                    '#' => arr[[i]] = 1,
                    '.' => arr[[i]] = 0,
                    _ => unimplemented!("unsupport character {}", c),
                }
            }
            arr
        }
});

pub trait ImageTransformer<T> {
    fn original(&self) -> Array2<T>;
    fn rot90_clockwise(&self) -> Array2<T>;
    fn rot180_clockwise(&self) -> Array2<T>;
    fn rot270_clockwise(&self) -> Array2<T>;
    fn flip_vertical(&self) -> Array2<T>;
    fn flip_horizontal(&self) -> Array2<T>;
    fn flip_main_diagonal(&self) -> Array2<T>;
    fn flip_sub_diagonal(&self) -> Array2<T>;
}

impl<T> ImageTransformer<T> for Array2<T>
where
    T: Copy,
{
    fn original(&self) -> Array2<T> {
        self.clone()
    }
    fn rot90_clockwise(&self) -> Array2<T> {
        let mut arr = self.clone();
        arr.swap_axes(0, 1);
        arr.flip_horizontal()
    }
    fn rot180_clockwise(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.slice(s![..;-1, ..;-1]));
        arr
    }
    fn rot270_clockwise(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.slice(s![.., ..;-1]));
        arr.swap_axes(0, 1);
        arr
    }
    fn flip_vertical(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.slice(s![..;-1, ..]));
        arr
    }
    fn flip_horizontal(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.slice(s![.., ..;-1]));
        arr
    }
    fn flip_main_diagonal(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.t());
        arr
    }
    fn flip_sub_diagonal(&self) -> Array2<T> {
        let shape = self.shape()[0];
        let mut arr: Array2<T> = unsafe { Array2::uninitialized((shape, shape)) };
        arr.assign(&self.rot270_clockwise().t());
        arr.rot90_clockwise()
    }
}

#[allow(unused)]
#[derive(Eq)]
pub struct Tile {
    tile_id: usize,
    sub_image: Array2<u8>,
    borders: Vec<(u32, u32, u32, u32)>,
}

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        self.tile_id == other.tile_id
    }
}

use std::fmt::Debug;
impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.tile_id)?;
        Ok(())
    }
}

impl Tile {
    pub fn new(data: &str) -> Self {
        let lines = data
            .split('\n')
            .map(|s| s.trim_end().to_string())
            .collect::<Vec<_>>();
        let shape = lines[1].len() - 2;
        let tile_id = parse_tile::parse_tile_id(&lines[0]).unwrap();
        let (top, top_rev) = parse_tile::parse_border(&lines[1]).unwrap();
        let left_col = lines
            .iter()
            .skip(1)
            .map(|s| s.chars().next().unwrap())
            .collect::<String>();
        let (left, left_rev) = parse_tile::parse_border(&left_col).unwrap();
        let right_col = lines
            .iter()
            .skip(1)
            .map(|s| s.chars().last().unwrap())
            .collect::<String>();
        let (right, right_rev) = parse_tile::parse_border(&right_col).unwrap();
        let (bottom, bottom_rev) = parse_tile::parse_border(&lines[lines.len() - 1]).unwrap();
        let mut sub_image = unsafe { Array2::<u8>::uninitialized((shape, shape)) };
        for (i, row) in lines.iter().enumerate().skip(2).take(shape) {
            let row_pixels = parse_tile::parse_sub_image(&row[1..row.len() - 1]).unwrap();
            sub_image.row_mut(i - 2).assign(&row_pixels);
        }
        Self {
            tile_id,
            sub_image,
            borders: vec![
                (top, right, bottom, left),                 // original sub image
                (left_rev, top, right_rev, bottom),         // rotate 90 degree clockwise
                (bottom_rev, left_rev, top_rev, right_rev), // rotate 180 degree clockwise
                (right, bottom_rev, left, top_rev),         // rotate 270 degree clockwise
                (bottom, right_rev, top, left_rev),         // flip vertical
                (top_rev, left, bottom_rev, right),         // flip horizontal
                (left, bottom, right, top),                 // flip along main diagonal
                (right_rev, top_rev, left_rev, bottom_rev), // flip along sub diagonal
            ],
        }
    }

    pub fn get_sub_image(&self, idx: usize) -> Array2<u8> {
        match idx {
            0 => self.sub_image.original(),
            1 => self.sub_image.rot90_clockwise(),
            2 => self.sub_image.rot180_clockwise(),
            3 => self.sub_image.rot270_clockwise(),
            4 => self.sub_image.flip_vertical(),
            5 => self.sub_image.flip_horizontal(),
            6 => self.sub_image.flip_main_diagonal(),
            7 => self.sub_image.flip_sub_diagonal(),
            _ => unreachable!("not a valid form index: {}", idx),
        }
    }
}

pub struct BigImage {
    tiles: Vec<Tile>,
    shape: usize,
}

impl BigImage {
    pub fn new(tiles: Vec<Tile>) -> Self {
        let shape = (tiles.len() as f64).sqrt() as usize;
        Self { shape, tiles }
    }

    pub fn fits<'a>(
        &'a self,
        row: usize,
        col: usize,
        prev_images: &[(&'a Tile, usize)],
    ) -> Vec<(&'a Tile, usize)> {
        let mut result: Vec<(&Tile, usize)> = vec![];
        result.extend_from_slice(prev_images);
        for tile in self.tiles.iter() {
            if result.iter().any(|(t, _)| t == &tile) {
                continue;
            }
            result.push((tile, 0));
            let upper_tile = if row > 0 {
                Some(result[(row - 1) * self.shape + col])
            } else {
                None
            };
            let left_tile = if col > 0 {
                Some(result[row * self.shape + col - 1])
            } else {
                None
            };
            for idx in 0..8 {
                result.last_mut().unwrap().1 = idx;
                if (row == 0
                    || tile.borders[idx].0
                        == upper_tile.unwrap().0.borders[upper_tile.unwrap().1].2)
                    && (col == 0
                        || tile.borders[idx].3
                            == left_tile.unwrap().0.borders[left_tile.unwrap().1].1)
                {
                    if row == self.shape - 1 && col == self.shape - 1 {
                        return result;
                    }
                    let (new_row, new_col) = if col + 1 >= self.shape {
                        (row + 1, 0)
                    } else {
                        (row, col + 1)
                    };
                    let ret = self.fits(new_row, new_col, &result);
                    if !ret.is_empty() {
                        return ret;
                    }
                }
            }
            result.pop();
        }
        vec![]
    }

    pub fn splice_result(&self, fit_result: &[(&Tile, usize)]) -> Array2<u8> {
        let pixels = fit_result[0].0.sub_image.shape()[0];
        let mut big_image = Array2::<u8>::zeros((0, self.shape * pixels));
        for row in 0..self.shape {
            let mut row_image = Array2::<u8>::zeros((pixels, 0));
            for col in 0..self.shape {
                let result = fit_result[row * self.shape + col];
                row_image = concatenate![Axis(1), row_image, result.0.get_sub_image(result.1)];
            }
            big_image = concatenate![Axis(0), big_image, row_image];
        }
        big_image
    }
}

pub fn part1_solution(fit_result: &[(&Tile, usize)]) -> usize {
    let shape = (fit_result.len() as f64).sqrt() as usize;
    let corner_idx = &[0, shape - 1, shape * (shape - 1), shape * shape - 1];
    fit_result
        .iter()
        .enumerate()
        .filter(|(idx, _)| corner_idx.contains(idx))
        .map(|(_, (t, _))| t.tile_id)
        .product()
}

lazy_static! {
    static ref MONSTER: Array2<u8> = unsafe {
        Array2::from_shape_vec_unchecked(
            (3, 20),
            vec![
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0, 0, 0, 1, 1, 0,
                0, 0, 0, 1, 1, 0, 0, 0, 0, 1, 1, 1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0,
                1, 0, 0, 0,
            ],
        )
    };
}

fn find_all_monsters(image: &Array2<u8>) -> Vec<(usize, usize)> {
    let shape = image.shape()[0];
    let mut found = vec![];
    for row in 0..=shape - MONSTER.shape()[0] {
        for col in 0..=shape - MONSTER.shape()[1] {
            if &image.slice(s![
                row..row + MONSTER.shape()[0],
                col..col + MONSTER.shape()[1]
            ]) & &MONSTER.slice(s![.., ..])
                == MONSTER.slice(s![.., ..])
            {
                found.push((row, col));
            }
        }
    }
    found
}

pub fn part2_solution(big_image: &BigImage, fit_result: &[(&Tile, usize)]) -> usize {
    let mut image = big_image.splice_result(fit_result);
    let monsters_pos = find_all_monsters(&image);
    for (row, col) in monsters_pos {
        let region = &image.slice(s![
            row..row + MONSTER.shape()[0],
            col..col + MONSTER.shape()[1]
        ]) - &MONSTER.slice(s![.., ..]);
        image
            .slice_mut(s![
                row..row + MONSTER.shape()[0],
                col..col + MONSTER.shape()[1]
            ])
            .assign(&(region));
    }
    image.iter().map(|x| *x as usize).sum::<usize>()
}

pub fn read_input(input_file: &str) -> Vec<Tile> {
    std::fs::read_to_string(input_file)
        .unwrap()
        .split("\n\n")
        .filter(|&b| !b.trim().is_empty())
        .map(|b| Tile::new(b))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use ndarray::Array;

    #[test]
    fn test_matrix_transforms() {
        let m = Array::range(1., 5., 1.).into_shape((2, 2)).unwrap();
        assert_eq!(m.original(), ndarray::arr2(&[[1., 2.], [3., 4.]]));
        assert_eq!(m.rot90_clockwise(), ndarray::arr2(&[[3., 1.], [4., 2.]]));
        assert_eq!(m.rot180_clockwise(), ndarray::arr2(&[[4., 3.], [2., 1.]]));
        assert_eq!(m.rot270_clockwise(), ndarray::arr2(&[[2., 4.], [1., 3.]]));
        assert_eq!(m.flip_vertical(), ndarray::arr2(&[[3., 4.], [1., 2.]]));
        assert_eq!(m.flip_horizontal(), ndarray::arr2(&[[2., 1.], [4., 3.]]));
        assert_eq!(m.flip_main_diagonal(), ndarray::arr2(&[[1., 3.], [2., 4.]]));
        assert_eq!(m.flip_sub_diagonal(), ndarray::arr2(&[[4., 2.], [3., 1.]]));
    }

    #[test]
    fn test_part1() {
        let testcase = read_input("../testcase1.txt");
        let test_image = BigImage::new(testcase);
        let result = vec![];
        let result = test_image.fits(0, 0, &result);
        assert_eq!(part1_solution(&result), 20899048083289);
    }

    #[test]
    fn test_part2() {
        let testcase = read_input("../testcase1.txt");
        let test_image = BigImage::new(testcase);
        let result = vec![];
        let result = test_image.fits(0, 0, &result);
        assert_eq!(part2_solution(&test_image, &result), 273);
    }
}
