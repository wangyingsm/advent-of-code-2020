fn main() {
    let serie = &[8, 11, 0, 19, 1, 2];
    println!("Part I: {}", day15_rust::count_to_turn_array(serie, 2020));
    println!("Part II: {}", day15_rust::count_to_turn_array(serie, 30_000_000));
}
