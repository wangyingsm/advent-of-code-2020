pub fn read_input(input_file: &str) -> Vec<String> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.split("\n\n")
        .map(|s| s.trim_end().to_string())
        .collect()
}

use std::collections::HashSet;

pub fn part1_solution(groups: &[String]) -> usize {
    groups
        .iter()
        .map(|s| {
            let mut answers = HashSet::new();
            s.as_bytes()
                .iter()
                .filter(|&c| c.is_ascii_alphabetic())
                .for_each(|&c| {
                    answers.insert(c);
                });
            answers.len()
        })
        .sum()
}

pub fn part2_solution(groups: &[String]) -> usize {
    use reduce::Reduce;
    groups
        .iter()
        .map(|s| {
            s.lines()
                .map(|t| {
                    let mut answers = HashSet::new();
                    t.as_bytes().iter().for_each(|&c| {
                        answers.insert(c);
                    });
                    answers
                })
                .reduce(|x, y| x.intersection(&y).cloned().collect::<HashSet<_>>())
                .unwrap()
                .len()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let groups = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&groups), 11);
    }

    #[test]
    fn test_part2_solution() {
        let groups = read_input("../testcase1.txt");
        assert_eq!(part2_solution(&groups), 6);
    }
}
