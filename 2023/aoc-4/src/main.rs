use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();
    let input = fs::read_to_string("src/input.txt").expect("File read failure");

    let mut card_totals: Vec<u32> = Vec::new();
    card_totals.resize(input.lines().count(), 1);
    for (i, card) in input.lines().collect::<Vec<_>>().iter().enumerate() {
        if card_totals[i] == 0 {
            break;
        }
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

        let mut numbers_won = 0;
        for number in card_numbers {
            if number.trim() == "" {
                continue;
            }
            if winning_numbers.contains(&number) {
                numbers_won += 1;
            }
        }

        for card_i in (i + 1)..=(i + numbers_won) {
            card_totals[card_i] += card_totals[i];
        }
    }

    println!("Total cards are {}", card_totals.iter().sum::<u32>());
    println!("Time elapsed {}ms", now.elapsed().as_millis())
}
