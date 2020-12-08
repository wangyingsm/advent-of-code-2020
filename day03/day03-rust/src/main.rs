fn main() {
    let tree_map = day03_rust::read_input("../input.txt");
    let tree_map = tree_map.iter().map(AsRef::as_ref).collect::<Vec<_>>();
    println!("Part I: {}", day03_rust::part1_solution(&tree_map));
    println!("Part II: {}", day03_rust::part2_solution(&tree_map));
}