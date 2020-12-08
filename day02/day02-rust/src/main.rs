fn main() {
    let lines = day02_rust::read_input("../input.txt");
    let lines: Vec<&str> = lines.iter().map(AsRef::as_ref).collect();
    println!("Part I: {}", day02_rust::part1_solution(&lines));
    println!("Part II: {}", day02_rust::part2_solution(&lines));
}
