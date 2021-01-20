use std::ptr::NonNull;

#[derive(Debug)]
pub struct Cup {
    cup_id: u32,
    next: NonNull<Cup>,
    less_one: NonNull<Cup>,
    in_chain: bool,
}

#[derive(Debug)]
pub struct CupsCircle {
    current: NonNull<Cup>,
    number_1: NonNull<Cup>,
}

impl Drop for CupsCircle {
    fn drop(&mut self) {
        let current_ptr = self.current.as_ptr();
        let mut ptr = self.current.as_ptr();
        loop {
            unsafe {
                let next = (*ptr).next.as_ptr();
                std::ptr::drop_in_place(ptr);
                std::alloc::dealloc(ptr as *mut _, std::alloc::Layout::new::<Cup>());
                if next == current_ptr {
                    break;
                }
                ptr = next;
            }
        }
    }
}

impl<'a> CupsCircle {
    pub fn new(data: &str, end: u32) -> Self {
        let mut data_cups = Vec::with_capacity(data.len());
        for c in data.as_bytes().iter().map(|x| (x - b'0') as u32) {
            data_cups.push(Box::leak::<'a>(Box::new(Cup {
                // data_cups.push(Cup {
                cup_id: c,
                next: NonNull::dangling(),
                less_one: NonNull::dangling(),
                in_chain: true,
            })));
            // });
        }
        let current = &mut *data_cups[0] as *mut Cup;
        // let current = &mut data_cups[0] as *mut Cup;
        let mut prev_cup = &mut *data_cups[0] as *mut Cup;
        // let mut prev_cup = &mut data_cups[0] as *mut Cup;
        for cup in data_cups.iter_mut().skip(1) {
            (unsafe { &mut *prev_cup }).next = NonNull::new(*cup as *mut Cup).unwrap();
            // (unsafe { &mut *prev_cup }).next = NonNull::new(cup as *mut Cup).unwrap();
            prev_cup = *cup as *mut Cup;
            // prev_cup = cup as *mut Cup;
        }
        data_cups.sort_by_key(|x| x.cup_id);
        let number_1 = &mut *data_cups[0] as *mut Cup;
        // let number_1 = &mut data_cups[0] as *mut Cup;
        let mut less_one_cup = &mut *data_cups[0] as *mut Cup;
        // let mut less_one_cup = &mut data_cups[0] as *mut Cup;
        for cup in data_cups.iter_mut().skip(1) {
            cup.less_one = NonNull::new(less_one_cup).unwrap();
            less_one_cup = *cup as *mut Cup;
            // less_one_cup = cup as *mut Cup;
        }
        for cup_id in (data_cups.len() as u32 + 1)..=end {
            let cup = Box::leak::<'a>(Box::new(Cup {
                // let mut cup = Cup {
                cup_id,
                next: NonNull::dangling(),
                less_one: NonNull::new(less_one_cup).unwrap(),
                in_chain: true,
            }));
            // };
            (unsafe { &mut *prev_cup }).next = NonNull::new(&mut *cup as *mut Cup).unwrap();
            // (unsafe { &mut *prev_cup }).next = NonNull::new(&mut cup as *mut Cup).unwrap();
            prev_cup = &mut *cup as *mut Cup;
            // prev_cup = &mut cup as *mut Cup;
            less_one_cup = &mut *cup as *mut Cup;
            // less_one_cup = &mut cup as *mut Cup;
        }
        (unsafe { &mut *prev_cup }).next = NonNull::new(current).unwrap();
        (unsafe { &mut *number_1 }).less_one = NonNull::new(less_one_cup).unwrap();
        Self {
            current: NonNull::new(current).unwrap(),
            number_1: NonNull::new(number_1).unwrap(),
        }
    }

    pub fn next(&mut self) {
        self.current = unsafe { &*self.current.as_ptr() }.next;
    }

    pub fn take_tripple(&self) -> NonNull<Cup> {
        let tripple_begin = unsafe { &*self.current.as_ptr() }.next;
        let mut ptr = tripple_begin.as_ptr();
        for _ in 0..3 {
            unsafe { &mut *ptr }.in_chain = false;
            ptr = unsafe { &*ptr }.next.as_ptr();
        }
        unsafe {
            (*self.current.as_ptr()).next = NonNull::new(ptr).unwrap();
            // (*ptr).next = NonNull::dangling();
        }
        tripple_begin
    }

    pub fn find_position(&self) -> NonNull<Cup> {
        unsafe {
            let mut ptr = (*self.current.as_ptr()).less_one.as_ptr();
            while !(*ptr).in_chain {
                ptr = (*ptr).less_one.as_ptr();
            }
            NonNull::new(ptr).unwrap()
        }
    }

    pub fn put_tripple_back(&mut self, tripple_begin: NonNull<Cup>, position: NonNull<Cup>) {
        unsafe {
            let next = (*position.as_ptr()).next;
            (*position.as_ptr()).next = tripple_begin;
            let mut ptr = tripple_begin.as_ptr();
            for _ in 0..2 {
                (*ptr).in_chain = true;
                ptr = (*ptr).next.as_ptr();
            }
            (*ptr).in_chain = true;
            (*ptr).next = next;
        }
        self.next();
    }
}

pub fn part1_solution(data: &str, moves: usize) -> u32 {
    let mut circle = CupsCircle::new(data, 9);
    for _ in 0..moves {
        let tripple_begin = circle.take_tripple();
        let position = circle.find_position();
        circle.put_tripple_back(tripple_begin, position);
    }
    let mut ptr = (unsafe { &*circle.number_1.as_ptr() }).next.as_ptr();
    let mut result = 0;
    unsafe {
        while (*ptr).cup_id != 1 {
            result = result * 10 + (*ptr).cup_id;
            ptr = (*ptr).next.as_ptr();
        }
    }
    result
}

pub fn part2_solution(data: &str, end: u32, moves: usize) -> u64 {
    let mut circle = CupsCircle::new(data, end);
    for _ in 0..moves {
        let tripple_begin = circle.take_tripple();
        let position = circle.find_position();
        circle.put_tripple_back(tripple_begin, position);
    }
    let ptr = (unsafe { &*circle.number_1.as_ptr() }).next.as_ptr();
    unsafe { (*ptr).cup_id as u64 * (*(*ptr).next.as_ptr()).cup_id as u64 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1_solution("389125467", 10), 92658374);
        assert_eq!(part1_solution("389125467", 100), 67384529);
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2_solution("389125467", 1_000_000, 10_000_000),
            149245887792
        );
    }
}
