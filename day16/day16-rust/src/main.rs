fn main() {
    let (ranges, your_ticket, nearby_tickets) = day16_rust::read_input("../input.txt");
    println!(
        "Part I: {}",
        day16_rust::part1_solution(&nearby_tickets, &ranges)
    );
    println!(
        "Part II: {}",
        day16_rust::part2_solution(&ranges, &nearby_tickets, &your_ticket)
    );
}
