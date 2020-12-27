fn main() {
    let lines = day18_rust::read_input("../input.txt");
    println!(
        "Part I: {}",
        day18_rust::all_solutions(&lines, &mut day18_rust::part1_do_math)
    );
    println!(
        "Part II: {}",
        day18_rust::all_solutions(&lines, &mut day18_rust::part2_do_math)
    );
}
