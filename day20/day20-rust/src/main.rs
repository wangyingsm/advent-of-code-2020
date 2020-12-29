fn main() {
    let tiles = day20_rust::read_input("../input.txt");
    let big_image = day20_rust::BigImage::new(tiles);
    let result = vec![];
    let result = big_image.fits(0, 0, &result);
    println!("Part I: {}", day20_rust::part1_solution(&result));
    println!(
        "Part II: {}",
        day20_rust::part2_solution(&big_image, &result)
    );
}
