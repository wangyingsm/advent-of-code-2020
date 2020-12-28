fn main() {
    let (form_map, ing_list) = day21_rust::read_input("../input.txt");
    let aller_map = day21_rust::find_all_allergens(form_map);
    println!(
        "Part I: {}",
        day21_rust::part1_solution(&aller_map, &ing_list)
    );
    println!("Part II: {}", day21_rust::part2_solution(&aller_map));
}
