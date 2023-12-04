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

        for n in 0..row.len() {
            let character = row[n];
            let character_number = character.to_string().parse::<i32>();
            let character_is_number = character_number.is_ok();

            if !character_is_number && character != '.' {
                fn expand_number_from(grid: &Vec<Vec<char>>, y: usize, x: usize) -> i32 {
                    if x > grid.len() - 1 || y > grid[0].len() - 1 {
                        return -1;
                    };
                    let starting_char = grid[y][x];
                    if !starting_char.is_numeric() {
                        return -1;
                    };

                    let mut starting_number = starting_char.to_string().parse::<i32>().unwrap();
                    if x > 0 && grid[y][x - 1].is_numeric() {
                        starting_number += grid[y][x - 1].to_string().parse::<i32>().unwrap() * 10;
                        if grid[y][x - 2].is_numeric() {
                            starting_number +=
                                grid[y][x - 2].to_string().parse::<i32>().unwrap() * 100;
                        }
                    }
                    if x < grid[0].len() - 1 && grid[y][x + 1].is_numeric() {
                        starting_number = starting_number * 10
                            + grid[y][x + 1].to_string().parse::<i32>().unwrap();
                        if grid[y][x + 2].is_numeric() {
                            starting_number = starting_number * 10
                                + grid[y][x + 2].to_string().parse::<i32>().unwrap();
                        }
                    }

                    starting_number
                }

                let mut numbers: Vec<i32> = vec![];

                if i > 0 && n > 0 {
                    // Get number above
                    let number_above = expand_number_from(&character_grid, i - 1, n);
                    if number_above != -1 {
                        numbers.push(number_above)
                    } else if n > 1 && n < row.len() - 2 {
                        numbers.push(expand_number_from(&character_grid, i - 1, n - 1));
                        numbers.push(expand_number_from(&character_grid, i - 1, n + 1));
                    }
                }

                if i < character_grid.len() - 1 && n > 0 {
                    let number_below = expand_number_from(&character_grid, i + 1, n);
                    if number_below != -1 {
                        numbers.push(number_below)
                    } else if n > 1 && n < row.len() - 2 {
                        numbers.push(expand_number_from(&character_grid, i + 1, n - 1));
                        numbers.push(expand_number_from(&character_grid, i + 1, n + 1));
                    }
                }

                if n > 0 {
                    numbers.push(expand_number_from(&character_grid, i, n - 1));
                }
                if n < row.len() - 1 {
                    numbers.push(expand_number_from(&character_grid, i, n + 1));
                }

                numbers.retain(|&e| e != -1);

                if numbers.len() == 2 {
                    let ratio = numbers[0] * numbers[1];
                    total += ratio
                }
            }
        }
    }

    println!("Total part numbers: {}", total);
    println!("Time: {}ms", now.elapsed().as_millis());
}
