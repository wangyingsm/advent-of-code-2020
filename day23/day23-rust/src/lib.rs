use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Cup {
    cup_id: u32,
    next: RefCell<Option<Rc<RefCell<Cup>>>>,
    less_one: RefCell<Option<Rc<RefCell<Cup>>>>,
    in_chain: bool,
}

impl std::fmt::Display for Cup {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.cup_id)?;
        Ok(())
    }
}

pub struct CupsCircle {
    current: RefCell<Rc<RefCell<Cup>>>,
    number_1: RefCell<Rc<RefCell<Cup>>>,
}

impl CupsCircle {
    pub fn new(data: &str, end: u32) -> Self {
        let mut data_cups = Vec::with_capacity(data.len());
        for c in data.as_bytes().iter().map(|x| (x - b'0') as u32) {
            data_cups.push(RefCell::new(Rc::new(RefCell::new(Cup {
                cup_id: c,
                next: RefCell::new(None),
                less_one: RefCell::new(None),
                in_chain: true,
            }))));
        }
        let prev_cup = RefCell::new(Rc::clone(&*data_cups[0].borrow()));
        let current = RefCell::new(Rc::clone(&*prev_cup.borrow()));
        for cup in data_cups.iter().skip(1) {
            prev_cup
                .borrow()
                .borrow()
                .next
                .replace(Some(Rc::clone(&*cup.borrow())));
            prev_cup.replace(Rc::clone(&*cup.borrow()));
        }
        data_cups.sort_by_key(|x| x.borrow().borrow().cup_id);
        let less_one_cup = RefCell::new(Rc::clone(&*data_cups[0].borrow()));
        let number_1 = RefCell::new(Rc::clone(&*less_one_cup.borrow()));
        for cup in data_cups.iter().skip(1) {
            cup.borrow()
                .borrow()
                .less_one
                .replace(Some(Rc::clone(&*less_one_cup.borrow())));
            less_one_cup.replace(Rc::clone(&*cup.borrow()));
        }
        for cup_id in (data.len() + 1) as u32..=end {
            let cup = RefCell::new(Rc::new(RefCell::new(Cup {
                cup_id,
                next: RefCell::new(None),
                less_one: RefCell::new(Some(Rc::clone(&*less_one_cup.borrow()))),
                in_chain: true,
            })));
            prev_cup
                .borrow()
                .borrow()
                .next
                .replace(Some(Rc::clone(&*cup.borrow())));
            prev_cup.replace(Rc::clone(&*cup.borrow()));
            less_one_cup.replace(Rc::clone(&*cup.borrow()));
        }
        prev_cup
            .borrow()
            .borrow()
            .next
            .replace(Some(Rc::clone(&*current.borrow())));
        number_1
            .borrow()
            .borrow()
            .less_one
            .replace(Some(Rc::clone(&*less_one_cup.borrow())));
        Self { number_1, current }
    }

    pub fn next(&self) {
        let next = Rc::clone(
            &self
                .current
                .borrow()
                .borrow()
                .next
                .borrow()
                .as_ref()
                .unwrap(),
        );
        self.current.replace(next);
    }

    pub fn take_tripple(&self) -> RefCell<Rc<RefCell<Cup>>> {
        let tripple_begin = RefCell::new(Rc::clone(
            &*self
                .current
                .borrow()
                .borrow()
                .next
                .borrow()
                .as_ref()
                .unwrap(),
        ));
        let ptr = RefCell::new(Rc::clone(&*tripple_begin.borrow()));
        for _ in 0..2 {
            ptr.borrow().borrow_mut().in_chain = false;
            let next = Rc::clone(&ptr.borrow().borrow().next.borrow().as_ref().unwrap());
            ptr.replace(next);
        }
        ptr.borrow().borrow_mut().in_chain = false;
        let next = Rc::clone(&ptr.borrow().borrow().next.borrow().as_ref().unwrap());
        ptr.borrow().borrow().next.replace(None);
        self.current
            .borrow()
            .borrow()
            .next
            .replace(Some(Rc::clone(&next)));
        tripple_begin
    }

    pub fn find_position(&self) -> RefCell<Rc<RefCell<Cup>>> {
        let ptr = RefCell::new(Rc::clone(
            &self
                .current
                .borrow()
                .borrow()
                .less_one
                .borrow()
                .as_ref()
                .unwrap(),
        ));
        while !ptr.borrow().borrow().in_chain {
            let next = Rc::clone(&ptr.borrow().borrow().less_one.borrow().as_ref().unwrap());
            ptr.replace(next);
        }
        ptr
    }

    pub fn put_tripple_back(
        &self,
        tripple: &RefCell<Rc<RefCell<Cup>>>,
        position: &RefCell<Rc<RefCell<Cup>>>,
    ) {
        let ptr = RefCell::new(Rc::clone(&*position.borrow()));
        let pos_next = RefCell::new(Rc::clone(
            &position.borrow().borrow().next.borrow().as_ref().unwrap(),
        ));
        ptr.borrow()
            .borrow()
            .next
            .replace(Some(Rc::clone(&*tripple.borrow())));
        for _ in 0..3 {
            let next = Rc::clone(&ptr.borrow().borrow().next.borrow().as_ref().unwrap());
            ptr.replace(next);
            ptr.borrow().borrow_mut().in_chain = true;
        }
        // ptr.borrow().borrow_mut().in_chain = true;
        ptr.borrow()
            .borrow_mut()
            .next
            .replace(Some(Rc::clone(&*pos_next.borrow())));
        self.next();
    }
}

pub fn part1_solution(data: &str, moves: usize) -> u32 {
    let circle = CupsCircle::new(data, 9);
    for _ in 0..moves {
        let tripple = circle.take_tripple();
        let position = circle.find_position();
        circle.put_tripple_back(&tripple, &position);
    }
    let ptr = RefCell::new(Rc::clone(
        &circle
            .number_1
            .borrow()
            .borrow()
            .next
            .borrow()
            .as_ref()
            .unwrap(),
    ));
    let mut result = 0;
    while ptr.borrow().borrow().cup_id != 1 {
        result = result * 10 + ptr.borrow().borrow().cup_id;
        let next = Rc::clone(&ptr.borrow().borrow().next.borrow().as_ref().unwrap());
        ptr.replace(next);
    }
    result
}

pub fn part2_solution(data: &str, end: u32, moves: usize) -> u64 {
    let circle = CupsCircle::new(data, end);
    for _ in 0..moves {
        let tripple = circle.take_tripple();
        let position = circle.find_position();
        circle.put_tripple_back(&tripple, &position);
    }
    let next_to_number_1 = Rc::clone(
        circle
            .number_1
            .borrow()
            .borrow()
            .next
            .borrow()
            .as_ref()
            .unwrap(),
    );
    let result = next_to_number_1.borrow().cup_id as u64
        * next_to_number_1
            .borrow()
            .next
            .borrow()
            .as_ref()
            .unwrap()
            .borrow()
            .cup_id as u64;
    result
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
