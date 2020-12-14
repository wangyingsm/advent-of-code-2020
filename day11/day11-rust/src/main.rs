fn main() {
    let cells = day11_rust::read_input("../input.txt");
    println!("Part I: {}", day11_rust::part1_solution(&cells));
    println!("Part II: {}", day11_rust::part2_solution(&cells));
}
