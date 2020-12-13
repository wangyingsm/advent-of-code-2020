use std::convert::From;

pub struct InputData {
    eta: u64,
    buses: Vec<(u64, usize)>,
}

impl InputData {
    pub fn new(input_file: &str) -> Self {
        use std::fs::File;
        use std::io::Read;
        let mut file = File::open(input_file).expect("open input file error");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("read input file error");
        let mut lines = buf.lines();
        let eta = lines.next().unwrap().parse::<u64>().unwrap();
        let mut buses = Vec::new();
        for (index, bus) in lines.next().unwrap().split(',').enumerate() {
            if bus == "x" {
                continue;
            }
            buses.push((bus.parse::<u64>().unwrap(), index));
        }
        Self { eta, buses }
    }

    pub fn part1_solution(&self) -> u64 {
        let mut m_waittime = u64::MAX;
        let mut m_bus_id = 0;
        for (bus_id, _) in &self.buses {
            let waittime = (self.eta / bus_id + 1) * bus_id - self.eta;
            if waittime < m_waittime {
                m_waittime = waittime;
                m_bus_id = *bus_id;
            }
        }
        m_waittime * m_bus_id
    }

    pub fn part2_solution(&self) -> u64 {
        use std::collections::HashSet;
        let step = self.buses.iter().max_by_key(|(id, _)| id).unwrap();
        let (mut step, max_index) = (step.0, step.1);
        let mut timestamp = step;
        let max_id = step;
        let mut ids = HashSet::new();
        ids.insert(max_id);
        loop {
            let t = timestamp - max_index as u64;
            let mut pass = true;
            for (bus_id, bus_index) in self.buses.iter().filter(|(bus_id, _)| *bus_id != max_id) {
                if (t + *bus_index as u64) % bus_id == 0 {
                    if !ids.contains(bus_id) {
                        step *= bus_id;
                        ids.insert(*bus_id);
                    }
                } else {
                    pass = false
                }
            }
            if pass {
                return t;
            }
            timestamp += step;
        }
    }
}

impl From<&str> for InputData {
    fn from(value: &str) -> Self {
        let mut buses = Vec::new();
        for (index, bus_id) in value.split(',').enumerate() {
            if bus_id == "x" {
                continue;
            }
            buses.push((bus_id.parse::<u64>().unwrap(), index));
        }
        Self { eta: 0, buses }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let testcase = InputData::new("../testcase1.txt");
        assert_eq!(testcase.part1_solution(), 295);
    }

    #[test]
    fn test_part2() {
        let testcase = InputData::new("../testcase1.txt");
        assert_eq!(testcase.part2_solution(), 1068781);
        let testcase = InputData::from("17,x,13,19");
        assert_eq!(testcase.part2_solution(), 3417);
        let testcase = InputData::from("67,7,59,61");
        assert_eq!(testcase.part2_solution(), 754018);
        let testcase = InputData::from("67,x,7,59,61");
        assert_eq!(testcase.part2_solution(), 779210);
        let testcase = InputData::from("67,7,x,59,61");
        assert_eq!(testcase.part2_solution(), 1261476);
        let testcase = InputData::from("1789,37,47,1889");
        assert_eq!(testcase.part2_solution(), 1202161486);
    }
}
