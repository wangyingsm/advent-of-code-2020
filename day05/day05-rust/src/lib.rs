use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
pub struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    pub fn seat_id(&self) -> u32 {
        self.row * 8 + self.col
    }

    fn decode(code: &str) -> u32 {
        let code = code.as_bytes();
        let mut span = 1 << code.len();
        let mut limits = (0, span - 1);
        for &c in code {
            span /= 2;
            match c {
                b'F' | b'L' => limits = (limits.0, limits.1 - span),
                b'B' | b'R' => limits = (limits.0 + span, limits.1),
                _ => unreachable!("code should only contain `FBLR`"),
            }
        }
        limits.0
    }
}

impl From<&str> for Seat {
    fn from(code: &str) -> Self {
        let (fb, lr) = code.split_at(code.len()-3);
        Self {
            row: Self::decode(fb),
            col: Self::decode(lr),
        }
    }
}

pub fn read_input(input_file: &str) -> Vec<Seat> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read input file error");
    buf.lines().map(|s| Seat::from(s)).collect()
}

pub fn part1_solution(codes: &[Seat]) -> Option<u32> {
    codes.iter().map(|s| s.seat_id()).max()
}

pub fn part2_solution(codes: &[Seat]) -> Option<u32> {
    let mut ids = codes.iter().map(|s| s.seat_id()).collect::<Vec<_>>();
    ids.sort();
    let (min, max) = (*ids.first().unwrap(), *ids.last().unwrap());
    for i in min..=max {
        if let Err(_) = ids.binary_search(&i) {
            return Some(i);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_seat_row_col() {
        assert_eq!(Seat::from("FBFBBFFRLR"), Seat{ row: 44, col: 5 });
        assert_eq!(Seat::from("BFFFBBFRRR"), Seat{ row: 70, col: 7 });
        assert_eq!(Seat::from("FFFBBBFRRR"), Seat{ row: 14, col: 7 });
        assert_eq!(Seat::from("BBFFBBFRLL"), Seat{ row: 102, col: 4 });
    }

    #[test]
    fn test_seat_id() {
        assert_eq!(Seat::from("FBFBBFFRLR").seat_id(), 357);
        assert_eq!(Seat::from("BFFFBBFRRR").seat_id(), 567);
        assert_eq!(Seat::from("FFFBBBFRRR").seat_id(), 119);
        assert_eq!(Seat::from("BBFFBBFRLL").seat_id(), 820);
    }
}
