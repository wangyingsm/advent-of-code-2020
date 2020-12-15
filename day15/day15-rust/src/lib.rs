#![feature(test)]

extern crate test;

pub fn count_to_turn(serie: &[u32], turn: usize) -> u32 {
    use std::collections::HashMap;
    let mut l = serie.len();
    let mut last_number = serie[l - 1];
    let mut records = HashMap::new();
    for (i, &n) in serie[..l].iter().enumerate() {
        records.insert(n, i);
    }
    loop {
        if l == turn {
            return last_number;
        }
        if records.contains_key(&last_number) {
            let previous = *records.get(&last_number).unwrap();
            records.insert(last_number, l - 1);
            last_number = l as u32 - previous as u32 - 1;
        } else {
            records.insert(last_number, l - 1);
            last_number = 0;
        }
        l += 1;
    }
}

pub fn count_to_turn_array(serie: &[u32], turn: usize) -> u32 {
    let mut arr: Vec<Option<usize>> = vec![None; turn];
    let mut l = serie.len();
    let mut last_number = serie[l - 1];
    for (i, &n) in serie[..l].iter().enumerate() {
        arr[n as usize] = Some(i);
    }
    loop {
        if l == turn {
            return last_number;
        }
        if let Some(previous) = arr[last_number as usize] {
            arr[last_number as usize] = Some(l - 1);
            last_number = l as u32 - previous as u32 - 1;
        } else {
            arr[last_number as usize] = Some(l - 1);
            last_number = 0;
        }
        l += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn test_count_to_2020() {
        assert_eq!(count_to_turn(&[0, 3, 6], 2020), 436);
        assert_eq!(count_to_turn(&[1, 3, 2], 2020), 1);
        assert_eq!(count_to_turn(&[2, 1, 3], 2020), 10);
        assert_eq!(count_to_turn(&[1, 2, 3], 2020), 27);
        assert_eq!(count_to_turn(&[2, 3, 1], 2020), 78);
        assert_eq!(count_to_turn(&[3, 2, 1], 2020), 438);
        assert_eq!(count_to_turn(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    #[ignore]
    fn test_count_to_30m() {
        assert_eq!(count_to_turn(&[0, 3, 6], 30_000_000), 175594);
        assert_eq!(count_to_turn(&[1, 3, 2], 30_000_000), 2578);
        assert_eq!(count_to_turn(&[2, 1, 3], 30_000_000), 3544142);
        assert_eq!(count_to_turn(&[1, 2, 3], 30_000_000), 261214);
        assert_eq!(count_to_turn(&[2, 3, 1], 30_000_000), 6895259);
        assert_eq!(count_to_turn(&[3, 2, 1], 30_000_000), 18);
        assert_eq!(count_to_turn(&[3, 1, 2], 30_000_000), 362);
    }

    #[bench]
    #[ignore]
    fn bench_count_to_30m(b: &mut Bencher) {
        b.iter(|| count_to_turn(&[0, 3, 6], 30_000_000));
    }

    #[bench]
    fn bench_count_to_30m_array(b: &mut Bencher) {
        b.iter(|| count_to_turn_array(&[0, 3, 6], 30_000_000));
    }

    #[bench]
    fn bench_count_to_2020(b: &mut Bencher) {
        b.iter(|| count_to_turn(&[0, 3, 6], 2020));
    }

    #[bench]
    fn bench_count_to_2020_array(b: &mut Bencher) {
        b.iter(|| count_to_turn_array(&[0, 3, 6], 2020));
    }

    #[test]
    fn test_count_to_2020_array() {
        assert_eq!(count_to_turn_array(&[0, 3, 6], 2020), 436);
        assert_eq!(count_to_turn_array(&[1, 3, 2], 2020), 1);
        assert_eq!(count_to_turn_array(&[2, 1, 3], 2020), 10);
        assert_eq!(count_to_turn_array(&[1, 2, 3], 2020), 27);
        assert_eq!(count_to_turn_array(&[2, 3, 1], 2020), 78);
        assert_eq!(count_to_turn_array(&[3, 2, 1], 2020), 438);
        assert_eq!(count_to_turn_array(&[3, 1, 2], 2020), 1836);
    }

    #[test]
    #[ignore]
    fn test_count_to_30m_array() {
        assert_eq!(count_to_turn_array(&[0, 3, 6], 30_000_000), 175594);
        assert_eq!(count_to_turn_array(&[1, 3, 2], 30_000_000), 2578);
        assert_eq!(count_to_turn_array(&[2, 1, 3], 30_000_000), 3544142);
        assert_eq!(count_to_turn_array(&[1, 2, 3], 30_000_000), 261214);
        assert_eq!(count_to_turn_array(&[2, 3, 1], 30_000_000), 6895259);
        assert_eq!(count_to_turn_array(&[3, 2, 1], 30_000_000), 18);
        assert_eq!(count_to_turn_array(&[3, 1, 2], 30_000_000), 362);
    }
}
