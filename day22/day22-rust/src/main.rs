fn main() {
    let mut cards = day22_rust::SpaceCards::new(&day22_rust::read_input("../input.txt"));
    println!(
        "Part I: {}",
        day22_rust::all_solution(&mut cards, day22_rust::SpaceCards::part1_game)
    );
    let mut cards = day22_rust::SpaceCards::new(&day22_rust::read_input("../input.txt"));
    println!(
        "Part II: {}",
        day22_rust::all_solution(&mut cards, day22_rust::SpaceCards::part2_game)
    );
}
