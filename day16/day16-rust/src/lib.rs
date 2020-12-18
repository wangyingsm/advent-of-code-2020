use std::convert::From;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub struct Range {
    low: u32,
    high: u32,
}

impl From<&str> for Range {
    fn from(value: &str) -> Self {
        let mut limits = value.split('-');
        let low = limits.next().unwrap().parse::<u32>().unwrap();
        let high = limits.next().unwrap().parse::<u32>().unwrap();
        Self{low, high}
    }
}

impl Range {
    pub fn contains(&self, value: u32) -> bool {
        self.low <= value && value <= self.high
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct ValidRanges {
    field_name: String,
    ranges: Vec<Range>,
}

impl ValidRanges {
    pub fn new(field_name: &str) -> Self {
        Self {
            field_name: field_name.to_string(),
            ranges: vec![]
        }
    }

    pub fn add_range(&mut self, range: &str) {
        self.ranges.push(Range::from(range));
    }

    pub fn contains(&self, value: u32) -> bool {
        self.ranges.iter().any(|r| r.contains(value))
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
