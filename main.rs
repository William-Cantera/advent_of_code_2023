use std::collections::HashMap;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use regex::Regex;
use indexmap::IndexMap;
use num::integer::lcm;

const INPUT_PATH: &'static str = "C:\\Users\\wfcan\\advent_of_code_2023\\day1\\src\\input\\";
fn main() {
    //problem1();
    //problem2();
    //problem3();
    //problem4();
    //problem5();
    problem6();
    //problem7();
    //problem8();
    //problem11();
    //problem12();
    //problem13(); // Stuck
    //problem15();
    //problem16();
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

fn combine_numbers(numbers: Vec<i64>) -> i64 {
    let mut combined = 0;
  
    for num in numbers {
      combined = combined * 10 + num;
    }
  
    combined
  }

fn problem5() {
    let file_path = INPUT_PATH.to_string() + "day3_input1.txt";

    if let Ok(file) = File::open(file_path) {
        let mut engine:Vec<Vec<i64>> = Vec::new();
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut engine_row: Vec<i64> = Vec::new();
                //println!("{:?}", line.chars().count());
                for char in line.chars() {
                    if char == '.' {
                        engine_row.push(-1);
                    } else if char.is_digit(10) {
                        engine_row.push(char.to_digit(10).unwrap() as i64);
                    } else {
                        engine_row.push(-2);
                    }
                }
                //println!("{:?}, {:?}", engine_row.len(), engine_row);
                //println!("");
                engine.push(engine_row);
            }
        }
        let symbol: i64 = -2;
        let period: i64 = -1;
        let mut total_part_numbers: i64 = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut str_num: String = String::from("");
        for _ in 0..engine.len() {
            for _ in 0..engine[i].len() {                              
                let val: i64 = engine[i][j];
                let mut is_part_num: bool = false;
                if val >= 0 {            
                    if i > 0 {
                        if j > 0 {
                            if engine[i-1][j-1] == symbol {
                                is_part_num = true;
                            }
                        } 
                        if engine[i-1][j] == symbol {
                            is_part_num = true;
                        } 
                        if j < engine[i].len() - 1 {
                            if engine[i-1][j+1] == symbol {
                                is_part_num = true;
                            }
                        }                   
                    }
                    if j > 0 {
                        if engine[i][j-1] == symbol {
                            is_part_num = true;
                        }
                    }
                    if j < engine[i].len() - 1 {
                        if engine[i][j+1] == symbol {
                            is_part_num = true;
                        }
                    }
                    if i < engine.len() - 1 {
                        if j > 0 {
                            if engine[i+1][j-1] == symbol {
                                is_part_num = true;
                            }
                        }
                        if engine[i+1][j] == symbol {
                            is_part_num = true;
                        }
                        if j < engine[i].len() - 1 {
                            if engine[i+1][j+1] == symbol {
                                is_part_num = true;
                            }
                        }
                    }
                    str_num.push_str(&val.to_string());               
                } else {
                    str_num = String::from("");
                }
                
                if is_part_num {
                    let mut eval: i64 = val;              
                    while !(eval == symbol || eval == period) {
                        if j+1 >= engine[i].len() {
                            break;
                        }
                        j += 1;
                                               
                        eval = engine[i][j];
                        if eval == symbol || eval == period {
                            break;
                        }
                        str_num.push_str(&eval.to_string());
                    }
                    total_part_numbers += str_num.parse::<i64>().unwrap();
                    println!("{:?}, j val {}", str_num, j);
                    str_num = String::from("");

                } else {
                    if j+1 >= engine[i].len() {
                        break;
                    }
                    j += 1;               
                }
            }
            i += 1;
            j = 0;
        }   // tried 540212
        println!("Problem 5 answer: {:?}", total_part_numbers);
    } else {
        println!("Error opening the file");
    }
}

fn problem6() {
    let file_path = INPUT_PATH.to_string() + "day3_input1.txt";

    if let Ok(file) = File::open(file_path) {
        let mut engine:Vec<Vec<i64>> = Vec::new();
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file);
        let star = -3;
        let symbol: i64 = -2;
        let period: i64 = -1;
        for line in reader.lines() {
            if let Ok(line) = line {
                let mut engine_row: Vec<i64> = Vec::new();
                //println!("{:?}", line.chars().count());
                for char in line.chars() {
                    if char == '*' {
                        engine_row.push(star);                   
                    } else if char == '.' {
                        engine_row.push(period);
                    } else if char.is_digit(10) {
                        engine_row.push(char.to_digit(10).unwrap() as i64);
                    } else {
                        engine_row.push(symbol);
                    }
                }
                engine.push(engine_row);
            }
        }
        let mut sum_of_gear_ratios: i64 = 0;
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut str_num1: String = String::from("");
        let mut str_num2: String = String::from("");
        for _ in 0..engine.len() {
            
            println!("Line: {:?} -------------", i+1);
            if i > 40 {
                //break;
            }

            for _ in 0..engine[i].len() {                              
                let val: i64 = engine[i][j];
                let mut part_num_coords: Vec<(usize, usize)> = Vec::new();
                let mut num_part_coords: i64 = 0;
                if val == star {            
                    if i > 0 {
                        if j > 0 {
                            if engine[i-1][j-1] >= 0 {
                                if !(engine[i-1][j] >= 0) {
                                    num_part_coords += 1;
                                }
                                part_num_coords.push((i - 1, j - 1));
                            }
                        } 
                        if engine[i-1][j] >= 0 {
                            num_part_coords += 1;
                            if part_num_coords.len() == 0 {
                                part_num_coords.push((i - 1, j));
                            }
                        } 
                        if j < engine[i].len() - 1 {
                            if engine[i-1][j+1] >= 0 && engine[i-1][j] < 0  {
                                if !(engine[i-1][j] >= 0) {
                                    num_part_coords += 1;
                                };
                                part_num_coords.push((i - 1, j + 1));
                            }
                        }                   
                    }
                    if j > 0 {
                        if engine[i][j-1] >= 0 {
                            num_part_coords += 1;
                            part_num_coords.push((i, j - 1));
                        }
                    }
                    if j < engine[i].len() - 1 {
                        if engine[i][j+1] >= 0 {
                            num_part_coords += 1;
                            part_num_coords.push((i, j + 1));
                        }
                    }
                    if i < engine.len() - 1 {
                        if j > 0 {
                            if engine[i+1][j-1] >= 0 {
                                if !(engine[i+1][j] >= 0) {
                                    num_part_coords += 1;
                                }
                                part_num_coords.push((i + 1, j - 1));
                            }
                        }
                        if engine[i+1][j] >= 0 {
                            num_part_coords += 1;
                            if part_num_coords.len() == 1 {
                                part_num_coords.push((i + 1, j));
                            }
                        }
                        if j < engine[i].len() - 1 {
                            if engine[i+1][j+1] >= 0 && engine[i+1][j] < 0 {
                                if !(engine[i+1][j] >= 0) {
                                    num_part_coords += 1;
                                }
                                part_num_coords.push((i + 1, j + 1));
                            }
                        }
                    }
                }                
                if part_num_coords.len() == 2 && num_part_coords == 2 {
                    let num1_i = part_num_coords[0].0;
                    let num1_j = part_num_coords[0].1;
                    let num2_i = part_num_coords[1].0;
                    let num2_j = part_num_coords[1].1;

                    str_num1.push_str(&engine[num1_i][num1_j].to_string());
                    str_num2.push_str(&engine[num2_i][num2_j].to_string());

                    if (num1_j-1) >= 0 as usize {           
                        if engine[num1_i][num1_j-1] >= 0 {
                            str_num1.insert_str(0, &engine[num1_i][num1_j-1].to_string());
                            if (num1_j-2) >= 0 as usize { 
                                if engine[num1_i][num1_j-2] >= 0 {
                                    str_num1.insert_str(0, &engine[num1_i][num1_j-2].to_string());
                                }
                            }
                        } 
                    } 
                    if num1_j+1 < engine[i].len() {
                        if engine[num1_i][num1_j+1] >= 0 {
                            str_num1.push_str(&engine[num1_i][num1_j+1].to_string());
                            if (num1_j+2) < engine[i].len() { 
                                if engine[num1_i][num1_j+2] >= 0 {
                                    str_num1.push_str(&engine[num1_i][num1_j+2].to_string());
                                }
                            }
                        }
                    }


                    if (num2_j-1) >= 0 as usize {           
                        if engine[num2_i][num2_j-1] >= 0 {
                            str_num2.insert_str(0, &engine[num2_i][num2_j-1].to_string());
                            if (num2_j-2) >= 0 as usize { 
                                if engine[num2_i][num2_j-2] >= 0 {
                                    str_num2.insert_str(0, &engine[num2_i][num2_j-2].to_string());
                                }
                            }
                        } 
                    } 
                    if num1_j+1 < engine[i].len() {
                        if engine[num2_i][num2_j+1] >= 0 {
                            str_num2.push_str(&engine[num2_i][num2_j+1].to_string());
                            if (num2_j+2) < engine[i].len() { 
                                if engine[num2_i][num2_j+2] >= 0 {
                                    str_num2.push_str(&engine[num2_i][num2_j+2].to_string());
                                }
                            }
                        }
                    }  
                    sum_of_gear_ratios += str_num1.parse::<i64>().unwrap() * str_num2.parse::<i64>().unwrap();
                    println!("{:?}, {:?}", str_num1, str_num2);
                    str_num1 = String::from("");
                    str_num2 = String::from("");
                }                 
                j += 1;                             
            }
            i += 1;
            j = 0;
        } // 90621577 - 87605697
        println!("Problem 6 answer: {:?}", sum_of_gear_ratios);
    } else {
        println!("Error opening the file");
    }
}

fn problem7() {
    let file_path = INPUT_PATH.to_string() + "day4_input1.txt";

    if let Ok(file) = File::open(file_path) {
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file); 
        
        let mut win_lst: Vec<Vec<i32>> = Vec::new();
        let mut value_lst: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line { 
                let split: Vec<&str> = line.split(":").collect();
                let card: Vec<&str> = split[1].split("|").collect();
                //println!("{:?}", card);
                let winning_numbers: Vec<&str> = card[0].split(" ").collect();
                win_lst.push(winning_numbers.iter().map(|v| v.parse::<i32>().unwrap_or(-1)).collect());

                let player_numbers: Vec<&str> = card[1].split(" ").collect();
                value_lst.push(player_numbers.iter().map(|v| v.parse::<i32>().unwrap_or(-1)).collect());
                //println!("{:?}", win_map);
                //println!("{:?}", value_lst);
            } 
        }
        let mut total_points: Vec<i128> = Vec::new();
        for (index, values) in value_lst.iter().enumerate() {
            let win_vals = win_lst.get(index).unwrap();
            
            let mut points: i128 = 0;
            for val in values.iter() {
                if *val == -1i32 {
                    continue;
                }
                if win_vals.contains(val) {
                    if points == 0 {
                        points += 1;
                    } else {
                        points *= 2;
                    }
                    println!("{:?}, {:?}", val, points);
                }
            }
            total_points.push(points);
        }

        println!("Problem 7 answer: {:?}", total_points.iter().sum::<i128>());
    } else {
        println!("Error opening the file");
    }
}

fn problem8() {
    let file_path = INPUT_PATH.to_string() + "day4_input1.txt";

    if let Ok(file) = File::open(file_path) {
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file); 
        
        let mut win_lst: Vec<Vec<i32>> = Vec::new();
        let mut value_lst: Vec<Vec<i32>> = Vec::new();
        for line in reader.lines() {
            if let Ok(line) = line { 
                let split: Vec<&str> = line.split(":").collect();
                let card: Vec<&str> = split[1].split("|").collect();
                let winning_numbers: Vec<&str> = card[0].split(" ").collect();
                win_lst.push(winning_numbers.iter().map(|v| v.parse::<i32>().unwrap_or(-1)).collect());

                let player_numbers: Vec<&str> = card[1].split(" ").collect();
                value_lst.push(player_numbers.iter().map(|v| v.parse::<i32>().unwrap_or(-1)).collect());
            } 
        }
        let mut copies: Vec<i128> = Vec::new();
        for _ in 0..207 {    
            copies.push(1);        
        }

        
        for (index, values) in value_lst.iter().enumerate() {            
            let win_vals = win_lst.get(index).unwrap();
            let mut num_matches: i32 = 0;
            println!("win vals {:?}", win_vals);
            println!("val vals {:?}", values);

           
            for val in values.iter() {
                if *val == -1 {
                    continue;
                }
                if win_vals.contains(val) {
                    num_matches += 1;
                }
            }

            //println!("natches {:?}", num_matches);
    
            for c in 0..num_matches {
                let card_num = c as usize + index + 1;
                //println!("{:?}", copies[index]);
                copies[card_num] += copies[index];                               
            }
            //println!("{:?}", copies);

            //println!("--------------------------------");

            
        }


        println!("Problem 8 answer: {:?}", copies.iter().sum::<i128>());
    } else {
        println!("Error opening the file");
    }
}

fn problem11() {
    let race_map: HashMap<&str, i32> = HashMap::from([
        ("53", 313),
        ("89", 1090),
        ("76", 1214),
        ("98", 1201),
    ]);
    let mut win_counts: Vec<i32> = Vec::new();
    for (key, value) in race_map {
        println!("{}: {}", key, value);
        let total_time: i32 = key.parse::<i32>().unwrap();
        let mut possible_wins: i32 = 0;
        for i in 0..total_time {
            if i * (total_time-i) > value {
                possible_wins += 1;
            }
        }
        win_counts.push(possible_wins);
    }
    println!("Problem 11 solution: {}", win_counts.iter().product::<i32>());
}

fn problem12() {
    let time: i128 = 53897698;
    let distance: i128 = 313109012141201;
    let mut possible_wins: i128 = 0;
    for i in 0..time {
        if i * (time-i) > distance {
            possible_wins += 1;
        }
    }
    println!("Problem 12 solution: {}", possible_wins);
}

fn problem13() {
    let file_path = INPUT_PATH.to_string() + "day7_input1.txt";
    let mut win_orders: Vec<(i32, String, i32)> = Vec::new();

    if let Ok(file) = File::open(file_path) {
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file); 

        let mut hand_bid_map: HashMap<String, i32> = HashMap::new();
        for line in reader.lines() {
            if let Ok(line) = line { 
               let split: Vec<&str> = line.split(" ").collect();
               hand_bid_map.insert(split[0].to_string(), split[1].parse::<i32>().unwrap());
            }
        }
        for (cardset, bid) in hand_bid_map {
            // Case 1, 5 of a kind
            let chars: std::str::Chars<'_> = cardset.chars();
            let mut cards_map = IndexMap::new();
            for c in chars {
                if cards_map.contains_key(&c) { 
                    cards_map.insert(c, cards_map[&c] + 1);
                } else {
                    cards_map.insert(c, 1);
                }       
            }
            let mut num5 = 0;
            let mut num4 = 0;
            let mut num3 = 0;
            let mut num2 = 0;
            let mut card_types: Vec<i32> = Vec::new();
            for (_, value) in &cards_map {
                card_types.push(*value);
            }
            //print!("{:?}", card_types);
            for ctype in card_types {
                if ctype == 5 {
                    num5 += 1;
                } else if ctype == 4 {
                    num4 += 1;
                } else if ctype == 3 {
                    num3 += 1;
                } else if ctype == 2 {
                    num2 += 1;
                }         
            }

            if num5 > 0 {  // Five of a kind
                win_orders.push((7, cardset, bid));
            } else if num4 > 0 { // Four of a kind
                win_orders.push((6, cardset, bid));
            } else if num3 > 0 && num2 > 0 { // Full house
                win_orders.push((5, cardset, bid));
            } else if num3 == 1 { // Three of a kind
                win_orders.push((4, cardset, bid));
            } else if num3 == 0 && num2 == 2 { // Two pairs
                win_orders.push((3, cardset, bid));
            } else if num3 == 0 && num2 == 1 { // One pair
                win_orders.push((2, cardset, bid));
            } else {
                win_orders.push((1, cardset, bid));           
            }

            
        }

        win_orders.sort_by(|a, b| a.0.cmp(&b.0));
        println!("{:?}", win_orders);

        for i in 0..(win_orders.len()-1) {
            let tuple1 = &win_orders[i];
            let tuple2 = &win_orders[i+1];
            if tuple1.0 == tuple2.0 {
                let tuple1_chars = tuple1.1.chars();
                println!("{}, {}", tuple1.1, tuple2.1);

                for (j, tuple1_char) in tuple1_chars.enumerate() {
                    let tuple2_char = tuple2.1.chars().nth(j).unwrap(); 
                    if tuple1_char == tuple2_char {
                        //println!("here1");
                        continue
                    }
                    if tuple2_char.is_digit(10) && tuple1_char.is_digit(10) && tuple2_char < tuple2_char {
                        //println!("not here");
                        win_orders.swap(i, i+1);
                        break;
                    } else if tuple2_char == 'K' && tuple1_char == 'A' {
                        //println!("here2");
                        win_orders.swap(i, i+1);
                        break;
                    }  else if tuple2_char == 'Q' && (tuple1_char == 'A' || tuple1_char == 'K') {
                        win_orders.swap(i, i+1);
                        break;
                    }  else if tuple2_char == 'J' && (tuple1_char == 'A' || tuple1_char == 'K' || tuple1_char == 'Q') {
                        win_orders.swap(i, i+1);
                        break;
                    }  else if tuple2_char == 'T' && (tuple1_char == 'A' || tuple1_char == 'K' || tuple1_char == 'Q' || tuple1_char == 'J') {
                        //println!("swap i {}", i);
                        win_orders.swap(i, i+1);
                        break;
                    } 
                }     
            }
        } 
        println!("{:?}", win_orders);


        let mut total = 0;
        for (i, tup) in win_orders.iter().enumerate() {
            total += tup.2 * (i+1) as i32;
        }
        println!("Problem 13 solution: {}", total);
    }
}

fn problem15() {
    let directions = "LLRRRLRLLRLRRLRLRLRRRLLRRLRRRLRRRLRRRLRRRLRRRLRRLRLLRRRLRRLLRLRLLLRRLRRLRLRLRLRRRLRLRRRLRRLLLRRRLLRRLLRRLLRRRLLLLRLRLRRRLRLRRRLRLLLRLRRLRRRLRRRLRRRLRRRLLRRLLLLRRLLRRLLRRLRLRRRLRRRLRRRLRRLRRRLRRLRRLRRLRLRRRLRRLRRRLRRRLRRLRLRRRLRRLLRLRRLRRRLRLRRLRRRLRRLRRLRRRLLRRRR";

    let file_path = INPUT_PATH.to_string() + "day8_input1.txt";

    if let Ok(file) = File::open(file_path) {
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file); 

        let mut directions_map: HashMap<String, (String, String)> = HashMap::new();
        for line in reader.lines() {
            if let Ok(line) = line { 
               let split: Vec<&str> = line.split("=").collect();
               
               let mut right_side = split[1].trim();
               right_side = &right_side[1..right_side.len()-1];
               let right_side_split: Vec<&str> = right_side.split(",").collect();

               directions_map.insert(split[0].trim().to_string(), (right_side_split[0].trim().to_string(), right_side_split[1].trim().to_string()));
            }
        }
        let mut key: &str = "AAA";
        let mut num_steps = 0;
        while key != "ZZZ" {
            for direction in directions.chars() {
                num_steps += 1;
                if direction == 'L' {
                    key = directions_map.get(key).unwrap().0.as_str();
                } else if direction == 'R' {
                    key = directions_map.get(key).unwrap().1.as_str();  
                } 
               
                if key == "ZZZ" {
                    break;
                }               
            }
        }
        println!("Problem 15 solution: {}", num_steps);
    }
}

fn get_steps(key: String, directions: &str, directions_map: &HashMap<String, (String, String)>) -> i128 {
    let mut num_steps: i128 = 0;
    let mut current_key = key;
    while !current_key.ends_with("Z") {
        for direction in directions.chars() {        
            num_steps += 1;
            if direction == 'L' {
                current_key = directions_map[&current_key].0.clone();
            } else if direction == 'R' {
                current_key = directions_map[&current_key].1.clone();
            }
            if current_key.ends_with("Z") {
                break;
            }
        }
    }

    num_steps
}

fn problem16() {
    let directions = "LLRRRLRLLRLRRLRLRLRRRLLRRLRRRLRRRLRRRLRRRLRRRLRRLRLLRRRLRRLLRLRLLLRRLRRLRLRLRLRRRLRLRRRLRRLLLRRRLLRRLLRRLLRRRLLLLRLRLRRRLRLRRRLRLLLRLRRLRRRLRRRLRRRLRRRLLRRLLLLRRLLRRLLRRLRLRRRLRRRLRRRLRRLRRRLRRLRRLRRLRLRRRLRRLRRRLRRRLRRLRLRRRLRRLLRLRRLRRRLRLRRLRRRLRRLRRLRRRLLRRRR";
    let file_path = INPUT_PATH.to_string() + "day8_input1.txt";
    if let Ok(file) = File::open(file_path) {
        // Read the file line by line
        let reader: BufReader<File> = BufReader::new(file); 

        let mut directions_map: HashMap<String, (String, String)> = HashMap::new();
        for line in reader.lines() {
            if let Ok(line) = line { 
               let split: Vec<&str> = line.split("=").collect();
               
               let mut right_side = split[1].trim();
               right_side = &right_side[1..right_side.len()-1];
               let right_side_split: Vec<&str> = right_side.split(",").collect();

               directions_map.insert(split[0].trim().to_string(), (right_side_split[0].trim().to_string(), right_side_split[1].trim().to_string()));
            }
        }
        let a_keys: Vec<String> = directions_map.iter()
        .filter(|(k, v)| k.ends_with('A'))
        .map(|(k, _)| k.to_string())
        .collect::<Vec<_>>();  
        
        let mut steps_for_keys: Vec<i128> = Vec::new();
        for key in a_keys {
            let steps = get_steps(key, directions, &directions_map);
            steps_for_keys.push(steps);
        }
        
        let mut result = steps_for_keys[0];
        for &num in &steps_for_keys[1..] {
            result = lcm(result, num);
        }
        println!("Problem 16 solution: {}", result);
    }
}
