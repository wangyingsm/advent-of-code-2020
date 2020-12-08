fn main() {
    let mut s = day01_rust::read_input("../input.txt");
    println!(
        "Part I solution 1: {}",
        day01_rust::part1_solution1(&s).unwrap()
    );
    println!(
        "Part I solution 2: {}",
        day01_rust::part1_solution2(&mut s).unwrap()
    );
    println!("Part II: {}", day01_rust::part2_solution(&s).unwrap());
}
