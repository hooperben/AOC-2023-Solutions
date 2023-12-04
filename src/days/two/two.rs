use crate::{
    days::two::one::{check_and_remove_semicolon, split_game_string},
    helpers::get_input::*,
};

#[allow(dead_code)]
pub fn run() {
    // batter up
    let input = get_input("src/days/two/input.txt");

    // split up by line
    let lines: Vec<&str> = input.split("\n").collect();

    // need to store a map per line
    let mut line_to_colour_record: std::collections::HashMap<
        String,
        std::collections::HashMap<&str, i32>,
    > = std::collections::HashMap::new();

    let mut line_count = 1;

    for line in lines.iter() {
        let mut colour_record = std::collections::HashMap::new();

        let line_split: Vec<String> = split_game_string(line);

        for part in line_split.iter().skip(1) {
            let game = check_and_remove_semicolon(&part);
            let game_split: Vec<&str> = game.split(",").collect();

            for current_game in game_split.iter() {
                let current_game = if current_game.starts_with(" ") {
                    &current_game[1..]
                } else {
                    current_game
                };

                let cc_split = current_game.split(" ").collect::<Vec<&str>>();

                let amount_int = cc_split[0].parse::<i32>().unwrap();
                let colour = cc_split[1].to_string();

                let current_amount = *colour_record.get(&colour.as_str()).unwrap_or(&0);

                if current_amount > amount_int {
                    // don't do anything
                } else {
                    if colour == "red" {
                        colour_record.insert("red", amount_int);
                    } else if colour == "green" {
                        colour_record.insert("green", amount_int);
                    } else {
                        colour_record.insert("blue", amount_int);
                    }
                }
            }
        }

        // line_count is our index here - will be the same as the game number
        line_to_colour_record.insert(line_count.to_string(), colour_record);
        line_count += 1;
    }

    let mut running_square_sum = 0;
    for line_to_colour_record in line_to_colour_record.iter() {
        // if all of these fields are not -1, it means that its a valid game
        if (line_to_colour_record.1.get("red").unwrap_or(&0) != &-1)
            && (line_to_colour_record.1.get("green").unwrap_or(&0) != &-1)
            && (line_to_colour_record.1.get("blue").unwrap_or(&0) != &-1)
        {
            running_square_sum += line_to_colour_record.1.get("red").unwrap_or(&0)
                * line_to_colour_record.1.get("green").unwrap_or(&0)
                * line_to_colour_record.1.get("blue").unwrap_or(&0);
        }
    }

    println!("{}", running_square_sum);
}
