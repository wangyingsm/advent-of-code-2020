use std::convert::From;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum CellStatus {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, Eq, PartialEq)]
pub struct SeatsMap {
    cells: Vec<Vec<CellStatus>>,
    cells_bak: Vec<Vec<CellStatus>>,
}

impl From<&str> for SeatsMap {
    fn from(input: &str) -> Self {
        let cells = input
            .lines()
            .map(|s| {
                s.trim_end()
                    .as_bytes()
                    .iter()
                    .map(|&c| match c {
                        b'.' => CellStatus::Floor,
                        b'L' => CellStatus::Empty,
                        b'#' => CellStatus::Occupied,
                        _ => unimplemented!("not support"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();
        let cells_bak = cells.iter().cloned().collect();
        Self { cells, cells_bak }
    }
}

impl SeatsMap {
    pub fn part1_count_occupied(&self, row: usize, col: usize) -> usize {
        let (row, col) = (row as i64, col as i64);
        let mut counter = 0;
        for dr in -1..2 {
            for dc in -1..2 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                let (r, c) = (row + dr, col + dc);
                if r < 0 || c < 0 || r >= self.cells.len() as i64 || c >= self.cells[0].len() as i64
                {
                    continue;
                }
                if self.cells[r as usize][c as usize] == CellStatus::Occupied {
                    counter += 1
                }
            }
        }
        counter
    }

    pub fn next_round(
        &mut self,
        f: &impl Fn(&SeatsMap, usize, usize) -> usize,
        thresh: usize,
    ) -> bool {
        let mut changed = false;
        for (row, line) in self.cells.iter().enumerate() {
            for (col, cell) in line.iter().enumerate() {
                let occupied = f(self, row, col);
                match (cell, occupied) {
                    (CellStatus::Empty, 0) => {
                        self.cells_bak[row][col] = CellStatus::Occupied;
                        changed = true;
                    }
                    (CellStatus::Occupied, _) if occupied >= thresh => {
                        self.cells_bak[row][col] = CellStatus::Empty;
                        changed = true;
                    }
                    _ => self.cells_bak[row][col] = self.cells[row][col],
                }
            }
        }
        std::mem::swap(&mut self.cells, &mut self.cells_bak);
        changed
    }

    pub fn count_all_occupied(&self) -> usize {
        self.cells
            .iter()
            .map(|r| r.iter().filter(|&&c| c == CellStatus::Occupied).count())
            .sum::<usize>()
    }

    pub fn part2_count_occupied(&self, row: usize, col: usize) -> usize {
        let mut counter = 0;
        for dr in -1..2 {
            for dc in -1..2 {
                if dr == 0 && dc == 0 {
                    continue;
                }
                counter += self.count_one_direction(row, col, dr, dc);
            }
        }
        counter
    }

    pub fn count_one_direction(&self, row: usize, col: usize, init_dr: i64, init_dc: i64) -> usize {
        let (mut dr, mut dc) = (init_dr, init_dc);
        loop {
            let r = row as i64 + dr;
            let c = col as i64 + dc;
            if r < 0 || c < 0 || r >= self.cells.len() as i64 || c >= self.cells[0].len() as i64 {
                return 0;
            }
            match self.cells[r as usize][c as usize] {
                CellStatus::Empty => return 0,
                CellStatus::Occupied => return 1,
                CellStatus::Floor => (),
            }
            dr += init_dr;
            dc += init_dc;
        }
    }
}

pub fn read_input(input_file: &str) -> String {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf
}

pub fn part1_solution(cells: &str) -> usize {
    let mut seats_map = SeatsMap::from(cells);
    loop {
        if !seats_map.next_round(&SeatsMap::part1_count_occupied, 4) {
            return seats_map.count_all_occupied();
        }
    }
}

pub fn part2_solution(cells: &str) -> usize {
    let mut seats_map = SeatsMap::from(cells);
    loop {
        if !seats_map.next_round(&SeatsMap::part2_count_occupied, 5) {
            return seats_map.count_all_occupied();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_clone() {
        let mut testcase = SeatsMap::from(&read_input("../testcase1.txt")[..]);
        assert_eq!(testcase.cells, testcase.cells_bak);
        testcase.cells[1][1] = CellStatus::Occupied;
        assert_ne!(testcase.cells, testcase.cells_bak);
        std::mem::swap(&mut testcase.cells, &mut testcase.cells_bak);
        assert_eq!(testcase.cells_bak[1][1], CellStatus::Occupied);
        std::mem::swap(&mut testcase.cells, &mut testcase.cells_bak);
        testcase.cells_bak[1][1] = CellStatus::Occupied;
        assert_eq!(testcase.cells, testcase.cells_bak);
    }

    #[test]
    fn test_part1() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&testcase), 37);
    }

    #[test]
    fn test_part2() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(part2_solution(&testcase), 26);
    }
}
