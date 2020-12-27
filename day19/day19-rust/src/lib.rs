use std::collections::HashMap;

#[derive(Debug)]
pub enum RuleChain {
    End(char),
    Chain(Vec<Vec<usize>>),
}

#[derive(Debug)]
pub struct Rule {
    rule_id: usize,
    rules: RuleChain,
}

peg::parser!( grammar parse_rule() for str {
    rule number() -> usize
        = n:$(['0'..='9']+) { n.parse().unwrap() }

    rule rule_id() -> usize
        = n:number() ": " { n }

    rule rule_id_list() -> Vec<usize>
        = l:number() ** " " { l }

    rule rule_chain() -> Vec<Vec<usize>>
        = ril:rule_id_list() ** " | " { ril }

    pub rule rule_end() -> Rule
        = id:rule_id() "\"" end:$(['a'..='z' | 'A'..='Z']) "\"" {
            Rule{
                rule_id: id,
                rules: RuleChain::End(end.chars().next().unwrap())
            }
        }

    pub rule rule_chains() -> Rule
        = id:rule_id() rc:rule_chain() {
            Rule {
                rule_id: id,
                rules: RuleChain::Chain(rc),
            }
        }
});

impl Rule {
    pub fn matches(&self, data: &str, all_rules: &HashMap<usize, Rule>) -> usize {
        match &self.rules {
            RuleChain::End(x) => {
                return if data.chars().next().unwrap() == *x {
                    1
                } else {
                    0
                }
            }
            RuleChain::Chain(rules) => {
                'outer: for forward in rules.iter() {
                    let mut index = 0;
                    for rule_id in forward {
                        let rule = all_rules.get(&rule_id).unwrap();
                        match rule.matches(&data[index..], all_rules) {
                            0 => continue 'outer,
                            x => index += x,
                        }
                    }
                    return index;
                }
            }
        }
        0
    }

    pub fn multiple_matches(&self, data: &str, all_rules: &HashMap<usize, Rule>) -> (usize, usize) {
        let mut index = self.matches(data, all_rules);
        let match_len = index;
        if index == 0 {
            return (0, 0);
        }
        let mut counter = 1;
        while index < data.len() {
            let ret = self.matches(&data[index..], all_rules);
            if ret == 0 {
                return (counter, match_len);
            }
            index += match_len;
            counter += 1;
        }
        (counter, match_len)
    }
}

pub fn read_input(input_file: &str) -> (HashMap<usize, Rule>, Vec<String>) {
    let parts = std::fs::read_to_string(input_file).unwrap();
    let mut parts = parts.split("\n\n");
    let all_rules = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| {
            let s = s.trim_end();
            let rule = if let Ok(r) = parse_rule::rule_chains(s) {
                r
            } else {
                parse_rule::rule_end(s).unwrap()
            };
            (rule.rule_id, rule)
        })
        .collect();
    let datas = parts
        .next()
        .unwrap()
        .lines()
        .map(|s| s.trim_end().to_string())
        .collect();
    (all_rules, datas)
}

pub fn part1_solution(all_rules: &HashMap<usize, Rule>, all_data: &[String]) -> usize {
    all_data
        .iter()
        .filter(|&x| all_rules.get(&0).unwrap().matches(&x[..], all_rules) == x.len())
        .count()
}

fn part2_matches(data: &str, all_rules: &HashMap<usize, Rule>) -> bool {
    let (m, m_match_len) = all_rules
        .get(&42)
        .unwrap()
        .multiple_matches(data, all_rules);
    if m == 0 {
        return false;
    }
    let (n, n_match_len) = if m * m_match_len == data.len() {
        all_rules
            .get(&31)
            .unwrap()
            .multiple_matches(&data[data.len() - m_match_len..], all_rules)
    } else {
        all_rules
            .get(&31)
            .unwrap()
            .multiple_matches(&data[m * m_match_len..], all_rules)
    };
    n > 0 && m > n && m * m_match_len + n * n_match_len == data.len()
}

pub fn part2_solution(all_rules: &HashMap<usize, Rule>, all_data: &[String]) -> usize {
    all_data
        .iter()
        .filter(|&s| part2_matches(s, all_rules))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (test_rules, _) = read_input("../testcase1.txt");
        assert_eq!(
            test_rules.get(&0).unwrap().matches("ababbb", &test_rules),
            6
        );
    }

    #[test]
    fn test_part2() {
        let (test_rules, test_data) = read_input("../testcase2.txt");
        assert_eq!(
            test_rules
                .get(&42)
                .unwrap()
                .multiple_matches("bbabbbbaabaabba", &test_rules),
            (2, 5),
        );
        assert!(!part2_matches("aaaabbaaaabbaaa", &test_rules));
        assert!(part2_matches(
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            &test_rules
        ));
        assert_eq!(part2_solution(&test_rules, &test_data), 12);
    }
}
