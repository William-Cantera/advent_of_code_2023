use std::collections::HashMap;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;

const INPUT_PATH: &'static str = "C:\\Users\\wfcan\\advent_of_code_2023\\day1\\src\\input\\";
fn main() {
    //problem1();
    //problem2();
    //problem3();
    problem4();
}

fn problem1() {
    // File path
    let file_path = INPUT_PATH.to_string() + "day1_input1.txt";

    // Open the file in read-only mode
    if let Ok(file) = File::open(file_path) {
        // Create a vector to store the input strings
        let mut lines: Vec<String> = Vec::new();

        // Read the file line by line
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }

        let mut nums: Vec<i32> = Vec::new();
        for line in lines {
            let mut is_first: bool = true;
            let mut char1 = 'x';
            let mut char2 = 'x'; 
            
            for c in line.chars() {
                if is_first && c.is_digit(10) {
                    char1 = c;
                    is_first = false;
                }
                if !is_first && c.is_digit(10) { 
                    char2 = c;
                }
            }
            let result: String;
            if char1.is_digit(10) && char2.is_digit(10) {
                result = format!("{}{}", char1.to_string(), char2.to_string());
            } else {
                result = char1.to_string();
            }
            nums.push(result.parse().expect("Not a number!"));
        }
        println!("Problem 1 answer: {:?}", nums.iter().sum::<i32>());
    } else {
        println!("Error opening the file");
    }
}

fn problem2() {
    // File path
    let file_path = INPUT_PATH.to_string() + "day1_input1.txt";

    // Open the file in read-only mode
    if let Ok(file) = File::open(file_path) {
        // Create a vector to store the input strings
        let mut lines: Vec<String> = Vec::new();

        // Read the file line by line
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                lines.push(line);
            }
        }

       let num_words = [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9")
        ];

        let mut sum: i32 = 0;
        for line in &lines {
            let mut output = line.to_owned();
            for (key, value) in &num_words {            
                output = output.replace(key, &format!("{}{}{}", key, value, key));
            }
            let digits: String = output.chars().filter(|c| c.is_digit(10)).collect();
            let first: char = digits.chars().next().unwrap();
            let last: char = digits.chars().rev().next().unwrap();
            let fl: String = format!("{}{}", first, last);
            println!("{:?}, {:?}, {:?}, {:?}", fl.parse::<i32>().unwrap(), fl, line, output);
            sum += fl.parse::<i32>().unwrap();
        }
    
        println!("Problem 2 answer: {:?}", sum);
    } else {
        println!("Error opening the file");
    }
}

fn problem3() {
    let file_path = INPUT_PATH.to_string() + "day2_input1.txt";

    if let Ok(file) = File::open(file_path) {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        let mut games_map: HashMap<i32, String> = HashMap::new();
        let mut sum_of_valid_games: i32 = 0;

        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<_> = line.split(":").collect();
                                
                if let Some(game_num) = Regex::new(r"(\d+)").unwrap().captures(values[0]) {
                    let number = game_num[1].parse::<i32>().unwrap();
                    games_map.insert(number, values[1].to_string());
                }
            }
        }

        for (key, value) in &games_map {
            //println!("map: {}: {}", key, value);
            let mut game_is_valid: bool = true;
            for cubes in value.split(";") {
                for picks in cubes.split(",") {
                    if let Some(pick) = Regex::new(r"(\d+)").unwrap().captures(picks) {
                        let pick_num = pick[1].parse::<i32>().unwrap();
                        if (picks.contains("red") && pick_num > max_red) 
                        || (picks.contains("green") && pick_num > max_green) 
                        || (picks.contains("blue") && pick_num > max_blue) {
                            game_is_valid = false;
                        } 
                    }                           
                }
            }
            if game_is_valid {
                sum_of_valid_games += key;                  
            }
        }
          
        println!("Problem 3 answer: {:?}", sum_of_valid_games);
    } else {
        println!("Error opening the file");
    }
}

fn problem4() {
    let file_path = INPUT_PATH.to_string() + "day2_input1.txt";

    if let Ok(file) = File::open(file_path) {
        let mut games_map: HashMap<i32, String> = HashMap::new();
        let mut sum_of_powers_of_minimally_valid_games: i32 = 0;

        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let values: Vec<_> = line.split(":").collect();
                                
                if let Some(game_num) = Regex::new(r"(\d+)").unwrap().captures(values[0]) {
                    let number = game_num[1].parse::<i32>().unwrap();
                    games_map.insert(number, values[1].to_string());
                }
            }
        }

        for (key, value) in &games_map {
            let mut min_greens = 0;
            let mut min_reds = 0;
            let mut min_blues = 0;
            for cubes in value.split(";") {
                for picks in cubes.split(",") {
                    if let Some(pick) = Regex::new(r"(\d+)").unwrap().captures(picks) {
                        let pick_num = pick[1].parse::<i32>().unwrap();
                        if picks.contains("red") && pick_num > min_reds {
                            min_reds = pick_num;
                        }
                        if picks.contains("green") && pick_num > min_greens {
                            min_greens = pick_num;
                        }
                        if picks.contains("blue") && pick_num >  min_blues { 
                            min_blues = pick_num;
                        }
                    }                           
                }
            }
            sum_of_powers_of_minimally_valid_games += (min_reds * min_greens * min_blues);
            println!("game: {:?}, sum: {:?}", key, sum_of_powers_of_minimally_valid_games);
        }             
        println!("Problem 4 answer: {:?}", sum_of_powers_of_minimally_valid_games);
    } else {
        println!("Error opening the file");
    }
}