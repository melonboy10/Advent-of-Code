use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("src/input.txt").expect("File read failure");

    let mut total = 0;
    for card in input.lines() {
        let winning_numbers = card.split("|").collect::<Vec<_>>()[0]
            .split(":")
            .collect::<Vec<_>>()[1]
            .trim()
            .split(" ")
            .collect::<Vec<_>>();
        let card_numbers = card.split("|").collect::<Vec<_>>()[1]
            .trim()
            .split(" ")
            .collect::<Vec<_>>();

        let mut score = 0;
        for number in card_numbers {
            if number.trim() == "" {
                continue;
            }
            if winning_numbers.contains(&number) {
                if score == 0 {
                    score = 1
                } else {
                    score *= 2;
                }
            }
        }
        total += score;
    }

    println!("Total scores are {}", total);
    println!("Time elapsed {}ms", now.elapsed().as_millis())
}
