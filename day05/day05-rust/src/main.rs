fn main() {
    let codes = day05_rust::read_input("../input.txt");
    println!("Part I: {}", day05_rust::part1_solution(&codes).unwrap());
    println!("Part II: {}", day05_rust::part2_solution(&codes).unwrap());
}
