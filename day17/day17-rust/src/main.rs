fn main() {
    let universe = day17_rust::read_input("../input.txt");
    println!(
        "Part I: {}",
        day17_rust::all_solutions::<day17_rust::Coordinate3D>(&universe, 3, 6)
    );
    println!(
        "Part II: {}",
        day17_rust::all_solutions::<day17_rust::Coordinate4D>(&universe, 4, 6)
    );
}
