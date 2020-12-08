#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Read;

pub fn read_input(input_file: &str) -> Vec<u64> {
    let mut file = File::open(input_file).expect("input file open error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.lines()
        .map(|s| s.trim_end().parse::<u64>().expect("wrong input line"))
        .collect::<Vec<_>>()
}

pub fn part1_solution1(serie: &[u64]) -> Result<u64, String> {
    for i in 0..serie.len() - 1 {
        for j in i + 1..serie.len() {
            if serie[i] + serie[j] == 2020 {
                return Ok(serie[i] * serie[j]);
            }
        }
    }
    Err("no such pair".to_string())
}

pub fn part1_solution2(serie: &mut [u64]) -> Result<u64, String> {
    serie.sort();
    let (mut i, mut j) = (0, serie.len() - 1);
    while i < j {
        match serie[i] + serie[j] {
            2020 => return Ok(serie[i] * serie[j]),
            x if x < 2020 => i += 1,
            _ => j -= 1,
        }
    }
    Err("no such pair".to_string())
}

pub fn part2_solution(serie: &[u64]) -> Result<u64, String> {
    let (mut i, mut j) = (0, serie.len() - 1);
    while i < j {
        while serie[i] + serie[j] >= 2020 {
            j -= 1;
        }
        for k in (i + 2..=j).rev() {
            let rem = 2020 - serie[i] - serie[k];
            if rem > serie[k] {
                break;
            }
            match serie[i + 1..k].binary_search(&rem) {
                Ok(_) => return Ok(serie[i] * serie[k] * rem),
                Err(_) => (),
            }
        }
        i += 1;
    }
    Err("no such triple".to_string())
}
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1_solution1(b: &mut Bencher) {
        let s = read_input("../input.txt");
        b.iter(|| part1_solution1(&s));
    }

    #[bench]
    fn bench_part1_solution2(b: &mut Bencher) {
        let mut s = read_input("../input.txt");
        b.iter(|| part1_solution2(&mut s));
    }

    #[bench]
    fn bench_par2_solution(b: &mut Bencher) {
        let mut s = read_input("../input.txt");
        &s.sort();
        b.iter(|| part2_solution(&s));
    }
}
