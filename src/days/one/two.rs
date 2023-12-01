use crate::helpers::get_input::*;

/**
 * Helper function - takes a string representation of a single digit number and returns that number
 */
fn string_to_number(s: &str) -> Option<u32> {
    match s {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None, // If the string does not match any number
    }
}

fn find_substrings_indices(s: &str) -> Vec<(&str, usize)> {
    // the sub strings we are looking for
    let substrings = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3", "4",
        "5", "6", "7", "8", "9",
    ];
    let mut results = Vec::new();

    for &substring in &substrings {
        let mut start = 0;
        while let Some(index) = s[start..].find(substring) {
            let adjusted_index = start + index;
            results.push((substring, adjusted_index));
            start = adjusted_index + 1;
        }
    }

    results.sort_by(|a, b| a.1.cmp(&b.1)); // sort the array by index
    results
}

#[allow(dead_code)]
pub fn run() {
    // batter up
    let input = get_input("src/days/one/input.txt");

    // split the input by lines
    let lines: Vec<&str> = input.split("\n").collect();

    // the running total for the whole file
    let mut running_total = 0;

    for line in lines.iter() {
        // get the indices of the number sub strings
        let arr = find_substrings_indices(line);

        // get a string representation of the current lines number
        let mut current_number = String::new();

        // get the first element of the substring array
        if let Some(first) = arr.first() {
            // if it's a digit, push it to the current_number string
            if first.0.chars().nth(0).unwrap().is_digit(10) {
                current_number.push(first.0.chars().nth(0).unwrap())
            } else {
                // else use our helper function to convert the string to a number and push that to the current_number string
                current_number.push_str(string_to_number(first.0).unwrap().to_string().as_str())
            }
        }

        // do the same for the last
        if let Some(last) = arr.last() {
            if last.0.chars().nth(0).unwrap().is_digit(10) {
                current_number.push(last.0.chars().nth(0).unwrap())
            } else {
                current_number.push_str(string_to_number(last.0).unwrap().to_string().as_str())
            }
        }

        // add the current number to the running total
        running_total += current_number.parse::<u32>().unwrap();
    }

    println!("{}", running_total);
}
