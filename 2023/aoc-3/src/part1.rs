use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file = "src/input.txt";
    let input = fs::read_to_string(input_file).expect("Unable to read file");
    let character_grid = input
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let mut total = 0;

    for i in 0..character_grid.len() {
        let row = &character_grid[i];
        let mut number = 0;
        let mut has_seen_star = false;

        for n in 0..row.len() {
            let character = row[n];
            let character_number = character.to_string().parse::<i32>();
            let character_is_number = character_number.is_ok();
            if !character_is_number {
                if has_seen_star {
                    total += number;
                }
                number = 0;
                has_seen_star = character != '.';
                continue;
            }

            number *= 10;
            number += character_number.unwrap();

            // Check around this number to set has seen star
            if i > 0 {
                if n > 0 && is_star(character_grid[i - 1][n - 1]) {
                    has_seen_star = true
                }
                if is_star(character_grid[i - 1][n + 0]) {
                    has_seen_star = true
                }
                if n < row.len() - 1 && is_star(character_grid[i - 1][n + 1]) {
                    has_seen_star = true
                }
            }
            if n > 0 && is_star(character_grid[i + 0][n - 1]) {
                has_seen_star = true
            }
            if n < row.len() - 1 && is_star(character_grid[i + 0][n + 1]) {
                has_seen_star = true
            }
            if i < character_grid.len() - 1 {
                if n > 0 && is_star(character_grid[i + 1][n - 1]) {
                    has_seen_star = true
                }
                if is_star(character_grid[i + 1][n + 0]) {
                    has_seen_star = true
                }
                if n < row.len() - 1 && is_star(character_grid[i + 1][n + 1]) {
                    has_seen_star = true
                }
            }
        }

        if has_seen_star {
            total += number;
        }
    }

    println!("Total part numbers: {}", total);

    println!("Time: {}ms", now.elapsed().as_millis());
}

fn is_star(character: char) -> bool {
    if character == '.' {
        return false;
    }
    let character_number = character.to_string().parse::<i32>();
    return character_number.is_err();
}
