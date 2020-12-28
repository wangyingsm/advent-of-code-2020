use std::collections::HashMap;

peg::parser!( grammar parse_formula() for str {
    rule parse_ingredients() -> Vec<&'input str>
        = ings:$(['a'..='z']+) ** " " { ings }

    rule parse_allergens() -> Vec<&'input str>
        = " (contains " allers:$(['a'..='z']+) ** ", " ")" { allers }

    pub rule parse_line() -> (Vec<&'input str>, Vec<&'input str>)
        = ings:parse_ingredients() allers:parse_allergens() { (ings, allers) }
});

type FormulaMap = HashMap<String, HashMap<String, usize>>;
type Ingredients = Vec<Vec<String>>;

pub fn read_input(input_file: &str) -> (FormulaMap, Ingredients) {
    let mut form_map: HashMap<String, HashMap<String, usize>> = HashMap::new();
    let mut ing_list: Vec<Vec<String>> = vec![];
    for line in std::fs::read_to_string(input_file).unwrap().lines() {
        let (ings, allers) = parse_formula::parse_line(line).unwrap();
        ing_list.push(ings.iter().map(|x| x.to_string()).collect());
        for a in allers {
            for i in ings.iter() {
                *form_map
                    .entry(a.to_string())
                    .or_default()
                    .entry(i.to_string())
                    .or_default() += 1;
            }
        }
    }
    (form_map, ing_list)
}

pub fn find_all_allergens(mut form_map: FormulaMap) -> HashMap<String, String> {
    let mut result = HashMap::new();
    let aller_count = form_map.len();
    while result.len() < aller_count {
        let mut map_clone = form_map.clone();
        for (aller, ing) in form_map {
            if result.contains_key(&aller[..]) {
                continue;
            }
            let mut kvs = ing
                .iter()
                .map(|(s, c)| (s.to_string(), c))
                .collect::<Vec<_>>();
            kvs.sort_by_key(|x| x.1);
            kvs.reverse();
            if kvs[0].1 > kvs[1].1 {
                map_clone.remove(&aller);
                result.insert(aller, kvs[0].0.clone());
                for (_, v) in map_clone.iter_mut() {
                    v.entry(kvs[0].0.clone()).and_modify(|e| *e = 0);
                }
            }
        }
        form_map = map_clone;
    }
    result
}

pub fn part1_solution(allergen_map: &HashMap<String, String>, ing_list: &[Vec<String>]) -> usize {
    let mut counter = 0;
    let mut ing_counted = vec![];
    for ings in ing_list {
        for i in ings {
            if allergen_map.values().any(|x| x == i) || ing_counted.contains(&i) {
                continue;
            }
            counter += ing_list.iter().filter(|v| v.contains(i)).count();
            ing_counted.push(i);
        }
    }
    counter
}

pub fn part2_solution(allergen_map: &HashMap<String, String>) -> String {
    let mut kvs: Vec<_> = allergen_map.iter().collect();
    kvs.sort_by_key(|&x| x.0);
    kvs.iter().map(|(_, v)| &v[..]).collect::<Vec<_>>()[..].join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (test_map, test_list) = read_input("../testcase1.txt");
        let test_aller_map = find_all_allergens(test_map);
        assert_eq!(part1_solution(&test_aller_map, &test_list), 5);
    }

    #[test]
    fn test_part2() {
        let (test_map, _) = read_input("../testcase1.txt");
        let test_aller_map = find_all_allergens(test_map);
        assert_eq!(part2_solution(&test_aller_map), "mxmxvkd,sqjhc,fvjkl");
    }
}
