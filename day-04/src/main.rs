mod reader;

fn main() {
    let data = "data/sample";
    // let data = "data/input";

    part_1(data);
    part_2(data);
}

fn part_1(dataset: &str) {
    let cards = reader::read_by_line(dataset).unwrap();
    let mut total = 0;
    for card in cards.iter() {
        total += get_card_points(card);
    }
    println!("Part 1: {total}");
}

fn part_2(dataset: &str) {
    println!("Part 2: {dataset}");
}

fn get_card_points(card: &String) -> i32 {
    let mut body = card.split(":").last().unwrap().split("|");
    let winning_numbers: Vec<&str> = body.next().unwrap().trim().split_whitespace().collect();
    let numbers: Vec<&str> = body.next().unwrap().trim().split_whitespace().collect();
    let mut points = 0;

    for number in numbers.iter() {
        if winning_numbers.contains(number) {
            if points < 1 {
                points = 1
            } else {
                points *= 2
            }
        }
    }
    points
}
