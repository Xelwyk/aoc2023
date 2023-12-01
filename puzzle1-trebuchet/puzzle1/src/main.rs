use core::panic;
use std::env;
use std::fs;

fn final_calibration_value(file_content: &String, include_words: bool) -> u32 {
    file_content.lines().map(|x: &str| get_line_calibration_value(x, include_words)).sum()
}

fn get_line_calibration_value(line: &str, include_words: bool) -> u32 {
    let mut first_number = get_first_number(line, false, include_words).to_string();
    let last_number = get_first_number(line, true, include_words).to_string();
    first_number.push_str(&last_number);
    let retval = first_number.parse().expect("cannot parse line calibration");
    retval
}

fn get_first_word_number(line: &str, numbers: &[String; 9]) -> (char, usize) {
    let window_size = if line.len() < 5 { line.len() } else { 5 };
    let mut window_start = 0;
    let mut window_end = window_start+window_size;
    let mut retval = '\0';
    
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

    //index of found word is used as search limit for numeric search pass, if we didn't
    //find any word we set limit to the end of the line
    if !retval.is_numeric() {
        window_start = line.len();
    }
    (retval, window_start)
}

fn get_first_number(input_line: &str, reverse_traverse: bool, include_words: bool) -> char {
    let mut line = input_line.to_lowercase();
    let mut retval = '\0';
    let mut search_limit = line.len();
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
        line = line.chars().rev().collect::<String>();
        numbers = numbers.map(|x| x.chars().rev().collect());
    }

    //removing mutability
    let line = line;
    let numbers = numbers;

    if include_words {
        (retval, search_limit) = get_first_word_number(line.as_str(), &numbers);
    }

    //removing mutability
    let retval = retval;
    let search_limit = search_limit;
        
    for (i, c) in line.chars().enumerate() {
        if i >= search_limit {
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
    let mut include_words = false;
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }
    if args.len() >= 3 && args[2].eq("-w") {
        include_words = true;
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("invalid file path");
    print!("{}", final_calibration_value(&content, include_words));
}
