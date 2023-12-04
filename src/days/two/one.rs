use crate::helpers::get_input::*;

// full credit to GPT for this bad boy but it took some good prompting I guess
pub fn split_game_string(input: &str) -> Vec<String> {
    let mut results = Vec::new();
    let mut current_str = String::new();
    let mut is_game_label = true; // to track if we are at the "Game X:" part

    for char in input.chars() {
        if char == ';' {
            if !current_str.trim().is_empty() {
                current_str.push(char);
                results.push(current_str.trim().to_string());
                current_str = String::new();
            }
            is_game_label = true;
            continue;
        }

        if char == ':' && is_game_label {
            current_str.push(char);
            results.push(current_str.trim().to_string());
            current_str = String::new();
            is_game_label = false;
            continue;
        }

        current_str.push(char);
    }

    if !current_str.trim().is_empty() {
        results.push(current_str.trim().to_string());
    }

    results
}

pub fn check_and_remove_semicolon(s: &str) -> String {
    let formatted: String;
    if s.ends_with(';') {
        formatted = s[..s.len() - 1].to_string()
    } else {
        formatted = s.to_string()
    }
    formatted
}

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

                if current_amount == -1 {
                    // do nothing - this game is already marked as invalid
                } else {
                    let possible_red = 12;
                    let possible_green = 13;
                    let possible_blue = 14;

                    let new_amount = current_amount + amount_int;

                    if colour == "red" {
                        if amount_int > possible_red {
                            colour_record.insert("red", -1);
                        } else {
                            colour_record.insert("red", new_amount);
                        }
                    } else if colour == "green" {
                        if amount_int > possible_green {
                            colour_record.insert("green", -1);
                        } else {
                            colour_record.insert("green", new_amount);
                        }
                    } else {
                        if amount_int > possible_blue {
                            colour_record.insert("blue", -1);
                        } else {
                            colour_record.insert("blue", new_amount);
                        }
                    }
                }
            }
        }

        // line_count is our index here - will be the same as the game number
        line_to_colour_record.insert(line_count.to_string(), colour_record);
        line_count += 1;
    }

    let mut valid_game_counter = 0;
    for line_to_colour_record in line_to_colour_record.iter() {
        // if all of these fields are not -1, it means that its a valid game
        if (line_to_colour_record.1.get("red").unwrap_or(&0) != &-1)
            && (line_to_colour_record.1.get("green").unwrap_or(&0) != &-1)
            && (line_to_colour_record.1.get("blue").unwrap_or(&0) != &-1)
        {
            valid_game_counter += line_to_colour_record.0.parse::<i32>().unwrap();
        }
    }

    println!("{}", valid_game_counter);
}
