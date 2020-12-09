#![feature(test)]

extern crate test;

use std::collections::HashMap;
use std::convert::From;

const REQUIRED_FIELDS: &[&str] = &["ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt"];
const EYE_COLORS: &[&str] = &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

pub struct Credential<'a> {
    fields: HashMap<&'a str, &'a str>,
}

impl<'a> From<&'a str> for Credential<'a> {
    fn from(value: &'a str) -> Self {
        let items = value.split_ascii_whitespace();
        let mut fields = HashMap::new();
        for item in items {
            let mut f = item.split(':');
            match (f.next(), f.next()) {
                (Some(k), Some(v)) => {
                    fields.insert(k, v);
                }
                _ => (),
            }
        }
        Self { fields }
    }
}

impl<'a> Credential<'a> {
    pub fn is_valid_part1(&self) -> bool {
        let key_len = self.fields.keys().count();
        (key_len == 7 || key_len == 8)
            && REQUIRED_FIELDS.iter().all(|k| self.fields.contains_key(k))
    }

    pub fn is_valid_part2(&self) -> bool {
        self.is_valid_part1()
            && self.verify_range("byr", 1920, 2002)
            && self.verify_range("iyr", 2010, 2020)
            && self.verify_range("eyr", 2020, 2030)
            && self.verify_hgt()
            && self.verify_ecl()
            && self.verify_hcl()
            && self.verify_pid()
    }

    fn verify_range(&self, range_key: &str, lower: u32, upper: u32) -> bool {
        let value = self
            .fields
            .get(&range_key)
            .unwrap()
            .parse::<u32>()
            .unwrap_or_default();
        lower <= value && value <= upper
    }

    fn verify_ecl(&self) -> bool {
        let value = self.fields.get(&"ecl").unwrap();
        EYE_COLORS.iter().any(|c| c == value)
    }

    fn verify_hgt(&self) -> bool {
        let height = self.fields.get(&"hgt").unwrap().as_bytes();
        let (height, unit) = height.split_at(height.len() - 2);
        match unit {
            &[b'c', b'm'] => {
                let height = height
                    .iter()
                    .map(|&x| x as char)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or_default();
                150 <= height && height <= 193
            }
            &[b'i', b'n'] => {
                let height = height
                    .iter()
                    .map(|&x| x as char)
                    .collect::<String>()
                    .parse::<u32>()
                    .unwrap_or_default();
                59 <= height && height <= 76
            }
            _ => false,
        }
    }

    fn verify_hcl(&self) -> bool {
        let color = self.fields.get("hcl").unwrap().as_bytes();
        if color.len() != 7 || color[0] != b'#' {
            return false;
        }
        color[1..]
            .iter()
            .any(|&c| (b'0' <= c && c <= b'9') || (b'a' <= c && c <= b'f'))
    }

    fn verify_pid(&self) -> bool {
        let pid = self.fields.get("pid").unwrap().as_bytes();
        if pid.len() != 9 {
            return false;
        }
        pid.iter().any(|&c| b'0' <= c && c <= b'9')
    }
}

pub fn read_input(input_file: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read input file error");
    buf.split("\n\n").map(|s| s.to_string()).collect::<Vec<_>>()
}

pub fn part1_solution(passports: &[&str]) -> usize {
    passports.iter().filter(|&&s| {
        let p = Credential::from(s);
        p.is_valid_part1()
    }).count()
}

pub fn part2_solution(passports: &[&str]) -> usize {
    passports.iter().filter(|&&s| {
        let p = Credential::from(s);
        p.is_valid_part2()
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_part1_solution(b: &mut Bencher) {
        let passports = read_input("../input.txt");
        let passports = passports.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
        b.iter(|| part1_solution(&passports));
    }

    #[bench]
    fn bench_part2_solution(b: &mut Bencher) {
        let passports = read_input("../input.txt");
        let passports = passports.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
        b.iter(|| part2_solution(&passports));
    }
}