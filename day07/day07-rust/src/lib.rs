#[derive(Debug, Eq, PartialEq)]
pub struct BagEdge {
    outer: String,
    inner: String,
    weight: u32,
}

impl BagEdge {
    pub fn new(outer: String, inner: String, weight: u32) -> Self {
        Self {
            outer,
            inner,
            weight,
        }
    }

    pub fn parse_verse(verse: &str) -> Result<(String, u32), ()> {
        if verse == "no other" {
            return Err(());
        }
        let mut parts = verse.split(' ');
        if let Ok(weight) = parts.next().unwrap().parse::<u32>() {
            let color = parts.collect::<String>();
            return Ok((color, weight));
        }
        return Ok((verse.split(' ').collect(), 1));
    }

    pub fn from_str(line: &str) -> Vec<BagEdge> {
        let line = line
            .replace(" bags.", "")
            .replace(" bags", "")
            .replace("bag.", "")
            .replace(" bag", "");
        let mut parts = line.split(" contain ");
        let (outer, outer_weight) = Self::parse_verse(parts.next().unwrap()).unwrap();
        let mut result = vec![];
        for i in parts.next().unwrap().split(", ") {
            if let Ok((inner, inner_weight)) = Self::parse_verse(i) {
                result.push(Self::new(outer.clone(), inner, inner_weight / outer_weight))
            }
        }
        result
    }

    pub fn inner(&self) -> &str {
        &self.inner
    }

    pub fn outer(&self) -> &str {
        &self.outer
    }

    pub fn weight(&self) -> u32 {
        self.weight
    }
}

pub fn read_input(input_file: &str) -> Vec<BagEdge> {
    use std::fs::File;
    use std::io::Read;
    let mut file = File::open(input_file).expect("open input file error");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("read input file error");
    buf.lines().map(|s| BagEdge::from_str(s)).fold(vec![], |mut x, y| {
        x.extend(y);
        x
    })
}

use std::collections::{HashMap, HashSet};
use std::rc::Rc;
use std::cell::RefCell;

fn search_outer_recursive<'a>(bags: &HashMap<&str, Vec<&'a BagEdge>>, inner_bag: &str, result: Rc<RefCell<HashSet<&'a str>>>) {
    if let Some(edges) = bags.get(inner_bag) {
        for e in edges.iter() {
            result.borrow_mut().insert(e.outer());
            search_outer_recursive(bags, e.outer(), Rc::clone(&result));
        }
    }
}

pub fn part1_solution(edges: &Vec<BagEdge>) -> usize {
    let mut bags: HashMap<&str, Vec<&BagEdge>> = HashMap::new();
    for e in edges {
        bags.entry(e.inner()).or_default().push(e);
    }
    let result = Rc::from(RefCell::from(HashSet::new()));
    search_outer_recursive(&bags, "shinygold", Rc::clone(&result));
    let result = result.borrow();
    result.len()
}

fn count_inner_recursive(bags: &HashMap<&str, Vec<&BagEdge>>, outer_bag: &str, init_qty: u32) -> u32 {
    let mut result = 0;
    if let Some(edges) = bags.get(outer_bag) {
        for e in edges.iter() {
            let qty = init_qty * e.weight();
            result += qty;
            result += count_inner_recursive(bags, e.inner(), qty);
        }
    }
    result
}

pub fn part2_solution(edges: &Vec<BagEdge>) -> u32 {
    let mut bags: HashMap<&str, Vec<&BagEdge>> = HashMap::new();
    for e in edges {
        bags.entry(e.outer()).or_default().push(e);
    }
    count_inner_recursive(&bags, "shinygold", 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_verse() {
        assert_eq!(
            BagEdge::parse_verse("1 bright white"),
            Ok(("brightwhite".to_string(), 1))
        );
        assert_eq!(
            BagEdge::parse_verse("6 dotted black"),
            Ok(("dottedblack".to_string(), 6))
        );
        assert_eq!(
            BagEdge::parse_verse("shiny gold"),
            Ok(("shinygold".to_string(), 1))
        );
        assert_eq!(BagEdge::parse_verse("no other"), Err(()));
    }

    #[test]
    fn test_from_str() {
        let expect = vec![
            BagEdge::new("lightred".to_string(), "brightwhite".to_string(), 1),
            BagEdge::new("lightred".to_string(), "mutedyellow".to_string(), 2),
        ];
        assert_eq!(
            BagEdge::from_str("light red bags contain 1 bright white bag, 2 muted yellow bags."),
            expect
        );
        let expect = vec![
            BagEdge::new("vibrantplum".to_string(), "fadedblue".to_string(), 5),
            BagEdge::new("vibrantplum".to_string(), "dottedblack".to_string(), 6),
        ];
        assert_eq!(
            BagEdge::from_str("vibrant plum bags contain 5 faded blue bags, 6 dotted black bags."),
            expect
        );
        let expect = vec![];
        assert_eq!(BagEdge::from_str("dotted black bags contain no other bags."), expect);
    }

    #[test]
    fn test_part1_solution() {
        let testcase1 = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&testcase1), 4);
    }

    #[test]
    fn test_part2_solution() {
        let testcase1 = read_input("../testcase1.txt");
        assert_eq!(part2_solution(&testcase1), 32);
        let testcase2 = read_input("../testcase2.txt");
        assert_eq!(part2_solution(&testcase2), 126);
    }
}
