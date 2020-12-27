fn main() {
    let (all_rules, all_data) = day19_rust::read_input("../input.txt");
    println!(
        "Part I: {}",
        day19_rust::part1_solution(&all_rules, &all_data)
    );
    println!(
        "Part II: {}",
        day19_rust::part2_solution(&all_rules, &all_data)
    );
}
