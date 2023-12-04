use std::fs;
use std::time::Instant;

fn main() {
    let start = Instant::now();

    println!("Hello, world!");
    const FILE_PATH: &str = "src/input.txt";

    let contents = fs::read_to_string(FILE_PATH).expect("Something went wrong reading the file");
    println!("Contents: {}", contents.len());

    let mut total = 0;
    for line in contents.split("\n") {
        let line_without_letters = line.replace(
            &[
                'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
                'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
            ][..],
            "",
        );

        let mut chars = line_without_letters.chars();
        let first_char = chars.next();
        let last_char = chars.last();

        let combined_letters: String = if first_char.is_some() && last_char.is_some() {
            format!("{}{}", first_char.unwrap(), last_char.unwrap())
        } else if first_char.is_some() {
            format!("{}{}", first_char.unwrap(), first_char.unwrap())
        } else {
            "0".to_string()
        };

        // println!("Number: {}", combined_letters);
        let line_as_number: i32 = combined_letters.parse().expect("Expecting a number");
        total += line_as_number;
    }

    println!("Total: {}", total);

    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
