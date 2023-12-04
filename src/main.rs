use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use regex::Regex;

const INPUT_PATH: &'static str = "C:\\Users\\wfcan\\AdventOfCode2023\\day1\\src\\input\\";

fn main() {
    problem1();
    problem2();
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
                //output = output.replace(key, value);
                //println!("{:?}", &format!("{}{}{}", key, value, key));
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