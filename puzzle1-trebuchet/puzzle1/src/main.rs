use core::panic;
use std::env;
use std::fs;

fn final_calibration_value(file_content: &String) -> u32 {
    file_content.lines().map(|x: &str| get_line_calibration_value(x)).sum()
}

fn get_line_calibration_value(line: &str) -> u32 {
    let mut first_number = get_first_number(line, false).to_string();
    let last_number = get_first_number(line, true).to_string();
    first_number.push_str(&last_number);
    let retval = first_number.parse().expect("cannot parse line calibration");
    retval
}

fn get_first_number(input_line: &str, reverse_traverse: bool) -> char {
    let mut line = input_line.to_lowercase();
    if reverse_traverse {
        line = line.chars().rev().collect::<String>();
    }
    let window_size = if line.len() < 5 { line.len() } else { 5 };
    let mut window_start = 0;
    let mut window_end = window_start+window_size;
    let mut retval = '\0';

    let mut numbers = [
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
    ];

    if reverse_traverse {
        numbers = numbers.map(|x| x.chars().rev().collect());
    }

    while window_end <= line.len() {
        for (number, word) in numbers.iter().enumerate() {
            match &line[window_start..window_end].find(word) {
                Some(x) => {
                    assert!(number+1 >= 1, "error: found word number is {}", number);
                    assert!(number+1 <= 9, "error: found word number is {}", number);
                    retval = char::from_digit(number as u32 + 1, 10).expect("error while parsing word index to char");
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
