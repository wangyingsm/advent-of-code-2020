use std::ops::RangeInclusive;

// 看了大神的博客后，决定尝试学习peg和nom，来解析输入文件
peg::parser! {
    grammar range_parser() for str {
        rule name() -> String
            = n:$(['a'..='z'|'A'..='Z'|' ']+) { n.into() }

        rule number() -> u32
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule range() -> RangeInclusive<u32>
            = m:number() "-" n:number() { m..=n }

        rule range_list() -> Vec<RangeInclusive<u32>>
            = l:range() ** " or " { l }

        pub rule to_valid_ranges() -> ValidRanges
            = rn:name() ": " l:range_list() { ValidRanges {
                field_name: rn,
                ranges: l,
            }}

        pub rule to_tickets() -> Vec<u32>
            = l:number() ** "," { l }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidRanges {
    field_name: String,
    ranges: Vec<RangeInclusive<u32>>,
}

impl ValidRanges {
    pub fn contains(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(&value))
    }
}

pub fn read_input(input_file: &str) -> (Vec<ValidRanges>, Vec<u32>, Vec<Vec<u32>>) {
    let contents = std::fs::read_to_string(input_file).unwrap();
    let mut contents = contents.split("\n\n");
    let ranges: Vec<ValidRanges> = contents
        .next()
        .unwrap()
        .lines()
        .map(range_parser::to_valid_ranges)
        .map(Result::unwrap)
        .collect();
    let your_ticket = {
        let mut your_ticket = contents.next().unwrap().lines();
        your_ticket.next().unwrap();
        range_parser::to_tickets(your_ticket.next().unwrap()).unwrap()
    };
    let nearby_tickets: Vec<Vec<u32>> = {
        let mut nearby_tickets = contents.next().unwrap().lines();
        nearby_tickets.next().unwrap();
        nearby_tickets
            .map(range_parser::to_tickets)
            .map(Result::unwrap)
            .collect()
    };
    (ranges, your_ticket, nearby_tickets)
}

pub fn part1_solution(nearby_tickets: &[Vec<u32>], valid_ranges: &[ValidRanges]) -> u32 {
    nearby_tickets
        .iter()
        .map(|v| {
            v.iter()
                .filter(|x| !valid_ranges.iter().any(|r| r.contains(**x)))
                .sum::<u32>()
        })
        .sum::<u32>()
}

fn get_all_valid_tickets<'a>(
    nearby_tickets: &'a [Vec<u32>],
    valid_ranges: &[ValidRanges],
) -> Vec<&'a [u32]> {
    let mut all_valid = vec![];
    for ticket in nearby_tickets {
        if ticket
            .iter()
            .all(|v| valid_ranges.iter().any(|r| r.contains(*v)))
        {
            all_valid.push(&ticket[..]);
        }
    }
    all_valid
}

fn get_fields_map<'a>(tickets: &[&[u32]], valid_ranges: &'a [ValidRanges]) -> Vec<Vec<&'a str>> {
    let mut fields_map = vec![];
    for i in 0..tickets[0].len() {
        let column: Vec<_> = tickets.iter().map(|&row| row[i]).collect();
        let mut names = vec![];
        for range in valid_ranges {
            let mut found = true;
            for x in column.iter() {
                if !range.contains(*x) {
                    found = false;
                    break;
                }
            }
            if found {
                names.push(&range.field_name[..]);
            }
        }
        fields_map.push(names);
    }
    fields_map
}

fn purge_fields_map(mut fields_map: Vec<Vec<&str>>) -> Vec<Vec<&str>> {
    loop {
        if fields_map.iter().all(|f| f.len() == 1) {
            return fields_map;
        }
        let to_purge: Vec<&str> = fields_map
            .iter()
            .filter(|&f| f.len() == 1)
            .map(|f| f[0])
            .collect();
        for fields in fields_map.iter_mut() {
            for purge in to_purge.iter() {
                if fields.contains(purge) && fields.len() > 1 {
                    let pos = fields.iter().position(|f| f == purge).unwrap();
                    fields.remove(pos);
                }
            }
        }
    }
}

pub fn part2_solution(
    ranges: &[ValidRanges],
    nearby_tickets: &[Vec<u32>],
    your_ticket: &[u32],
) -> u64 {
    let all_valids = get_all_valid_tickets(nearby_tickets, ranges);
    let mut fields_map = get_fields_map(&all_valids, ranges);
    fields_map = purge_fields_map(fields_map);
    let mut result = 1;
    for (i, f) in fields_map.iter().enumerate() {
        if f[0].starts_with("departure") {
            result *= your_ticket[i] as u64;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let (ranges, _, nearby_tickets) = read_input("../testcase1.txt");
        assert_eq!(part1_solution(&nearby_tickets, &ranges), 71);
    }
}
