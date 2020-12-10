fn main() {
    let bags = day07_rust::read_input("../input.txt");
    println!("Part I: {}", day07_rust::part1_solution(&bags));
    println!("Part II: {}", day07_rust::part2_solution(&bags));
}