pub fn read_input(input_file: &str) -> Vec<u32> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    let mut last_element = 0;
    let mut arr: Vec<_> = buf
        .lines()
        .map(|s| s.trim_end().parse::<u32>().unwrap())
        .collect();
    arr.sort();
    let mut arr: Vec<_> = arr
        .iter()
        .map(|&x| {
            let v = x - last_element;
            last_element = x;
            v
        })
        .collect();
    arr.push(3);
    arr
}

pub fn part1_solution(arr: &[u32]) -> usize {
    let mut counter_1 = 0;
    let mut counter_3 = 0;
    arr.iter().for_each(|&x| match x {
        1 => counter_1 += 1,
        3 => counter_3 += 1,
        _ => (),
    });
    counter_1 * counter_3
}

pub fn part2_solution(arr: &[u32]) -> u64 {
    let mut paths = 1;
    let mut counter = 0u64;
    for i in arr.iter() {
        match *i {
            1 => counter += 1,
            3 => {
                match counter {
                    x if x >= 3 => paths *= (counter - 3 + 1) * 3 + 1,
                    2 => paths *= 2,
                    _ => (),
                }
                counter = 0;
            }
            _ => unreachable!("invalid diff element"),
        }
    }
    paths
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let testcase1 = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&testcase1), 35);
        let testcase2 = read_input("../testcase2.txt");
        assert_eq!(part1_solution(&testcase2), 220);
    }

    #[test]
    fn test_part2_solution() {
        let testcase1 = read_input("../testcase1.txt");
        assert_eq!(part2_solution(&testcase1), 8);
        let testcase2 = read_input("../testcase2.txt");
        assert_eq!(part2_solution(&testcase2), 19208);
    }
}
