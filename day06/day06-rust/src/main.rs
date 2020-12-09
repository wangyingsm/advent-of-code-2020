fn main() {
    let groups = day06_rust::read_input("../input.txt");
    println!("Part I: {}", day06_rust::part1_solution(&groups));
    println!("Part II: {}", day06_rust::part2_solution(&groups));
}
