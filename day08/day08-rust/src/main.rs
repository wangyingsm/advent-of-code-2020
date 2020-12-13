fn main() {
    let mut pad = day08_rust::GamePad::new("../input.txt");
    let result2 = day08_rust::part2_solution(&pad).unwrap();
    println!("Part I: {}", day08_rust::part1_solution(&mut pad));
    println!("Part II: {}", result2);
}
