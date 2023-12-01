use std::collections::HashMap;
use std::fs;

/**
 * This function creates a new day file and adds it to the mod.rs file
 */
fn write_new_day_rs(day: &str) {
    let dir_path = format!("src/days/{}", day);
    // create the new directory
    match fs::create_dir(&dir_path) {
        Ok(_) => {}
        Err(e) => println!("Uh oh: {}", e),
    }

    // the input path for todays puzzle input
    let input_path = format!("src/days/{}/input.txt", day);
    let todays_code = format!(
        r#"
use crate::helpers::get_input::*;

#[allow(dead_code)]
pub fn run() {{
    // batter up
    let input = get_input("{}");

    println!("todays input: \n{{}}", input);
}}
"#,
        input_path
    );

    // write puzzle one
    let file_path = format!("src/days/{}/one.rs", day);
    match fs::write(&file_path, &todays_code) {
        Ok(_) => {}
        Err(e) => println!("Uh oh: {}", e),
    }

    // write puzzle two
    let file_path = format!("src/days/{}/two.rs", day);
    match fs::write(&file_path, &todays_code) {
        Ok(_) => {}
        Err(e) => println!("Uh oh: {}", e),
    }

    // also write to the mod.rs for it
    let mod_path = format!("pub mod one;\npub mod two;\n",);
    match fs::write(&format!("{}/mod.rs", dir_path), &mod_path) {
        Ok(_) => {}
        Err(e) => println!("Uh oh: {}", e),
    }
    // create an empty input.txt for the given day
    let input_path = format!("src/days/{}/input.txt", day);
    match fs::write(&input_path, "") {
        Ok(_) => {}
        Err(e) => println!("Uh oh: {}", e),
    }

    // Adding the new day to the root mod.rs file
    let mod_file_path = "src/days/mod.rs";

    match fs::read_to_string(mod_file_path) {
        Ok(contents) => {
            // if it doesn't already, otherwise catchhyyaaa
            if !contents.contains(&mod_path) {
                let mod_path = format!("pub mod {};\n", day);
                let new_contents = format!("{}\n{}", contents, mod_path);
                match fs::write(mod_file_path, new_contents) {
                    Ok(_) => {}
                    Err(e) => println!("Uh oh: {}", e),
                }
            }
        }
        Err(e) => println!("Uh oh: {}", e),
    }
}

/**
 * This function takes a day number as a string and returns the day as a word
 */
fn get_day_as_word(day: &str) -> String {
    let number_words = [
        ("1", "one"),
        ("2", "two"),
        ("3", "three"),
        ("4", "four"),
        ("5", "five"),
        ("6", "six"),
        ("7", "seven"),
        ("8", "eight"),
        ("9", "nine"),
        ("10", "ten"),
        ("11", "eleven"),
        ("12", "twelve"),
        ("13", "thirteen"),
        ("14", "fourteen"),
        ("15", "fifteen"),
        ("16", "sixteen"),
        ("17", "seventeen"),
        ("18", "eighteen"),
        ("19", "nineteen"),
        ("20", "twenty"),
        ("21", "twenty-one"),
        ("22", "twenty-two"),
        ("23", "twenty-three"),
        ("24", "twenty-four"),
        ("25", "twenty-five"),
    ]
    .iter()
    .cloned()
    .collect::<HashMap<_, _>>();
    // Your day variable as a string

    // Perform the lookup and store the result in day_as_word
    let day_as_word = match number_words.get(day) {
        Some(word) => word.to_string(),
        None => panic!("Bad day!! Please enter 1 - 25"), // or handle the error as you see fit
    };

    day_as_word
}

/**
 * This is a program that both handles:
 *
 * the creation of the new day e.g. 1  === one.rs
 * the insertion of this day in the mod.rs
 * and the modification of the main.rs file to add the new day as a runnable option
 */
pub fn run(day: &str) {
    // our root program to modify
    let file_path = "src/main.rs";

    // Read the file to a string
    match fs::read_to_string(file_path) {
        Ok(contents) => {
            // we want to just insert the current day above this line in main.rs
            let search_term = "_ => println!(\"re read the README.md perhaps?\"),";

            // Find our spot to insert the new day
            if contents.contains(search_term) {
                // get the day as a word
                let day_as_word = get_day_as_word(day);

                // check that this hasn't been inserted in main.rs already
                if contents.contains(&format!("{}::one::run();", day_as_word)) {
                    println!("{} is already in main.rs, doofus!!", day);
                    return;
                }

                let todays_code = format!(
                    r#"
            "{}" => {{
                if args[2].as_str() == "1" {{
                    {}::one::run();
                }} else {{
                    {}::two::run();
                }}
            }}
            {}
            "#,
                    day, day_as_word, day_as_word, search_term
                );

                // insert the new days runner above the search term
                let new_contents = contents.replace(search_term, &todays_code);

                // write the new day file and update mod.rs
                write_new_day_rs(&day_as_word);

                // write the new contents back to src/main.rs
                match fs::write(file_path, new_contents) {
                    Ok(_) => {}
                    Err(e) => println!("Uh oh: {}", e),
                }

                println!(
                    "day {} created, go and get it brussy/brussette",
                    &day_as_word
                );
            } else {
                println!("You've broken this, not me!");
            }
        }
        Err(e) => println!("Uh oh: {}", e),
    }
}
