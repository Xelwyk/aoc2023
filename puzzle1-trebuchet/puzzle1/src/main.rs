use core::panic;
use std::collections::HashMap;
use std::env;
use std::fs;

fn final_calibration_value(file_content: &String) -> u32 {
    file_content.lines().map(|x: &str| get_line_calibration_value(x)).sum()
}

fn get_line_calibration_value(line: &str) -> u32 {
    let mut first_number = get_first_number(line).to_string();
    let last_number = get_last_number(line).to_string();
    first_number.push_str(&last_number);
    let retval = first_number.parse().expect("cannot parse line calibration");
    //println!("{} | {}", line, retval);
    retval
}

fn get_first_number(input_line: &str) -> char {
    let line = input_line.to_lowercase();
    let window_size = if line.len() < 5 { line.len() } else { 5 };
    let mut window_start = 0;
    let mut window_end = window_start+window_size;
    let mut retval = '\0';
    let mut numbers = HashMap::new();
    numbers.insert("one", '1');
    numbers.insert("two", '2');
    numbers.insert("three", '3');
    numbers.insert("four", '4');
    numbers.insert("five", '5');
    numbers.insert("six", '6');
    numbers.insert("seven", '7');
    numbers.insert("eight", '8');
    numbers.insert("nine", '9');

    while window_end <= line.len() {
        let windowed_line = String::from(&line[window_start..window_end]);

        for (key, val) in &numbers {
            match windowed_line.find(key) {
                Some(x) => {
                    retval = *val;
                    window_start += x;
                    break;
                }
                _ => {},
            }
        }
        if retval.is_numeric() {
            break;
        }
        window_start += 1;
        window_end += 1;
    }

    //window_start is used as search limit for following for loop, if we didn't
    //find any word we set limit to the end of the line
    if !retval.is_numeric() {
        window_start = window_end;
    }

    for (i, c) in line.chars().enumerate() {
        if i >= window_start {
            break;
        }
        if c.is_numeric() {
            return c;
        }
    }

    if !retval.is_numeric() {
        panic!("no number found in line");
    }

    retval
}

fn get_last_number(input_line: &str) -> char {
    let line = input_line.to_lowercase().chars().rev().collect::<String>();
    let window_size = if line.len() < 5 { line.len() } else { 5 };
    let mut window_start = 0;
    let mut window_end = window_start+window_size;
    let mut retval = '\0';
    let mut numbers = HashMap::new();
    //it ain't stupid if it works
    numbers.insert("eno", '1');
    numbers.insert("owt", '2');
    numbers.insert("eerht", '3');
    numbers.insert("ruof", '4');
    numbers.insert("evif", '5');
    numbers.insert("xis", '6');
    numbers.insert("neves", '7');
    numbers.insert("thgie", '8');
    numbers.insert("enin", '9');

    while window_end <= line.len() {
        let windowed_line = String::from(&line[window_start..window_end]);

        for (key, val) in &numbers {
            match windowed_line.find(key) {
                Some(x) => {
                    retval = *val;
                    window_start += x;
                    break;
                }
                _ => {},
            }
        }
        if retval.is_numeric() {
            break;
        }
        window_start += 1;
        window_end += 1;
    }

    //window_start is used as search limit for following for loop, if we didn't
    //find any word we set limit to the end of the line
    if !retval.is_numeric() {
        window_start = window_end;
    }

    for (i, c) in line.chars().enumerate() {
        if i >= window_start {
            break;
        }
        if c.is_numeric() {
            return c;
        }
    }

    if !retval.is_numeric() {
        panic!("no number found in line");
    }
    retval
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("invalid file path");
    print!("{}", final_calibration_value(&content));
}
