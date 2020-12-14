fn main() {
    let instructions = day12_rust::read_input("../input.txt");
    println!("Part I: {}", day12_rust::part1_solution(&instructions));
    println!("Part II: {}", day12_rust::part2_solution(&instructions));
}
