use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file = "src/input.txt";
    let input = fs::read_to_string(input_file).expect("Unable to read file");

    let mut game_power_sum = 0;

    for line in input.split('\n') {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let game = line.split(':').collect::<Vec<_>>();
        let remaining_string = game[1].to_string();

        /* PART 2 */

        let mut minumum_red = 0;
        let mut minumum_green = 0;
        let mut minumum_blue = 0;

        for segment in remaining_string.split(";") {
            let color_segments = segment.split(",").collect::<Vec<_>>();
            for color in color_segments {
                let color_data = color.trim().split(" ").collect::<Vec<_>>();

                let color_variable = match color_data[1].trim() {
                    "red" => minumum_red,
                    "green" => minumum_green,
                    "blue" => minumum_blue,
                    _ => 0,
                };

                let color_value = color_data[0].parse::<i32>().unwrap();

                if color_value > color_variable {
                    match color_data[1].trim() {
                        "red" => minumum_red = color_value,
                        "green" => minumum_green = color_value,
                        "blue" => minumum_blue = color_value,
                        _ => (),
                    }
                }
            }
        }

        let game_power = minumum_red * minumum_green * minumum_blue;
        game_power_sum += game_power;
    }

    println!("Game Power Sum: {}", game_power_sum);

    println!("Time: {}ms", now.elapsed().as_millis());
}
