fn main() {
    let xmas = day09_rust::read_input("../input.txt");
    let result = day09_rust::find_first_invalid(&xmas, 25).unwrap();
    println!("Part I: {}", result);
    println!(
        "Part II: {}",
        day09_rust::find_contiguous_sum(&xmas, result).unwrap()
    );
}
