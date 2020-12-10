pub fn read_input(input_file: &str) -> Vec<u64> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("read input file error");
    buf.lines()
        .map(|s| s.trim_end().parse::<u64>().unwrap())
        .collect()
}

pub fn find_first_invalid(xmas: &[u64], preamble: usize) -> Option<u64> {
    'outer: for i in preamble..xmas.len() {
        let mut prev = xmas[i - preamble..i].iter().cloned().collect::<Vec<_>>();
        prev.sort();
        let (mut head, mut tail) = (0, prev.len() - 1);
        while head < tail {
            let s = prev[head] + prev[tail];
            if s == xmas[i] {
                continue 'outer;
            }
            if s < xmas[i] {
                head += 1;
            }
            if s > xmas[i] {
                tail -= 1;
            }
        }
        return Some(xmas[i]);
    }
    None
}

pub fn find_contiguous_sum(xmas: &[u64], expect: u64) -> Option<u64> {
    for span_len in 2..=xmas.len() {
        for win in xmas.windows(span_len) {
            if win.iter().sum::<u64>() == expect {
                return Some(win.iter().min().unwrap() + win.iter().max().unwrap());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_invalid() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(find_first_invalid(&testcase, 5), Some(127));
    }

    #[test]
    fn test_find_contiguous_sum() {
        let testcase = read_input("../testcase1.txt");
        assert_eq!(find_contiguous_sum(&testcase, 127), Some(62));
    }
}
