fn main() {
    let commands = day14_rust::read_input("../input.txt");
    println!("Part I: {}", day14_rust::part1_solution(&commands));
    println!("Part II: {}", day14_rust::part2_solution(&commands));
}
