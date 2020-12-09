fn main() {
    let passports = day04_rust::read_input("../input.txt");
    let passports = passports.iter().map(|s| s.as_ref()).collect::<Vec<_>>();
    println!("Part I: {}", day04_rust::part1_solution(&passports));
    println!("Part II: {}", day04_rust::part2_solution(&passports));
}