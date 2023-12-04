use std::fs;
use std::time::Instant;

fn main() {
    let now = Instant::now();

    let input_file = "src/input.txt";
    let input = fs::read_to_string(input_file).expect("Unable to read file");

    let total_red = 12;
    let total_green = 13;
    let total_blue = 14;

    let mut game_id_sum = 0;

    for line in input.split('\n') {
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        let game = line.split(':').collect::<Vec<_>>();
        let game_id = game[0].split(' ').collect::<Vec<_>>()[1]
            .parse::<i32>()
            .unwrap();
        let remaining_string = game[1].to_string();

        /* PART 1 */
        let mut game_possible = true;
        for segment in remaining_string.split(";") {
            let color_segments = segment.split(",").collect::<Vec<_>>();
            for color in color_segments {
                let color_data = color.trim().split(" ").collect::<Vec<_>>();

                println!("{} {}", color_data[0], color_data[1]);

                let color_variable = match color_data[1].trim() {
                    "red" => total_red,
                    "green" => total_green,
                    "blue" => total_blue,
                    _ => 0,
                };

                let color_value = color_data[0].parse::<i32>().unwrap();
                if color_value > color_variable {
                    game_possible = false;
                    break;
                }
            }

            if game_possible == false {
                break;
            }
        }

        if game_possible == true {
            game_id_sum += game_id;
        }
    }

    println!("Game ID Sum: {}", game_id_sum);

    println!("Time: {}ms", now.elapsed().as_millis());
}
