use crate::helpers::get_input::*;

#[allow(dead_code)]
pub fn run() {
    // batter up
    let input = get_input("src/days/one/input.txt");

    // split the input by lines
    let lines: Vec<&str> = input.split("\n").collect();

    // running total of the whole file
    let mut running_total = 0;

    for line in lines.iter() {
        // the current lines numbers as a string
        let mut current_number: String = String::new();

        // loop through the current line
        for c in line.chars() {
            // if it's a number - push it to the string
            if c.is_digit(10) {
                current_number.push(c);
            }
        }

        // this is a string that will just be the first and last character of the current_number string
        let mut final_number = String::new();

        // get the first
        if let Some(character) = current_number.chars().nth(0) {
            final_number.push(character);
        }

        // get the last
        if let Some(character) = current_number.chars().nth(current_number.len() - 1) {
            final_number.push(character);
        }

        // parse and add to the running total
        running_total += final_number.parse::<i32>().unwrap();
    }

    println!("{}", running_total);
}
