use std::env;
use std::fs;
struct Number {
    value:          u32,
    line_index:     usize,
    relative_start: usize,
    relative_end:   usize,
    star_adress:    Option<(usize, usize)>,
    multiplied:     bool,
}

fn sum_of_part_numbers(content: &String) -> u64 {
    get_part_numbers(content).iter().map(|n| u64::from(n.value)).sum()
}

fn get_gear_ratio_sum(content: &String) -> u32 {
    let mut sum = 0;
    let mut part_numbers = get_part_numbers(content);
    let mut gear_adressbook: Vec<usize> = vec![];
    for (index, number) in part_numbers.iter().enumerate() {
        match number.star_adress {
            Some(_) => { gear_adressbook.push(index) },
            None => {},
        }
    }
    for address in gear_adressbook {
        let number = &part_numbers[address];
        if number.multiplied {
            continue;
        }
        match multiply_gear(&mut part_numbers, address) {
            Some(product) => { sum += product },
            None => {},
        }
    }
    sum
}

fn multiply_gear(part_numbers: &mut Vec<Number>, initial_index: usize) -> Option<u32> {
    let mut index = initial_index+1;
    let star_address = part_numbers[initial_index].star_adress.expect("missing star address");
    while index < part_numbers.len() {
        match part_numbers[index].star_adress {
            Some(addr) => {
                if addr == star_address {
                    part_numbers[initial_index].multiplied = true;
                    part_numbers[index].multiplied = true;
                    return Some(part_numbers[initial_index].value * part_numbers[index].value)
                }
            },
            None => {},
        }
        index += 1;
    }
    return None
}

fn get_part_numbers(content: &String) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut part_numbers: Vec<Number> = vec![];
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        find_numbers(lines[i], i).iter().for_each(|n| 
            numbers.push(Number { ..*n })
        );
        i+=1;
    }
    numbers.iter_mut().for_each(|n|
        if establish_part_number(&lines,  n) { 
            part_numbers.push(Number { ..*n });
        }
    );
    part_numbers
}

fn find_numbers(line: &str, line_index: usize) -> Vec<Number> {
    let mut numbers: Vec<Number> = vec![];
    let mut stringy_number: String = String::new();
    let mut store_position_flag = false;
    let mut relative_start = 0;
    let mut relative_end = 0;

    for (i, element) in line.char_indices() {
        if element.is_numeric() {
            stringy_number.push(element);
            if !store_position_flag {
                relative_start = i;
                store_position_flag = !store_position_flag;
            }
        } else if !stringy_number.is_empty() {
            if store_position_flag {
                relative_end = i-1;
                store_position_flag = !store_position_flag;
            }
            let value = stringy_number.parse().expect("cannot parse stringified number");
            numbers.push(Number { value, relative_start, relative_end, line_index, star_adress: None, multiplied: false });
            stringy_number.clear();
        }
    }

    /*TODO: refactor duplicate code */
    if !stringy_number.is_empty() {
        let value = stringy_number.parse().expect("cannot parse stringified number");
        numbers.push(Number { value, relative_start, relative_end: line.len()-1, line_index, star_adress: None, multiplied: false });
    }
    numbers
}

fn establish_part_number(lines: &Vec<&str>, number: &mut Number) -> bool {
    if lines.is_empty() {
        return false;
    }
    let mut line_index = if number.line_index == 0 { 0 } else { number.line_index-1 };
    let line_index_stop = if number.line_index+1 >= lines.len() { lines.len()-1 } else { number.line_index+1 };
    
    while line_index <= line_index_stop {
        let line = *lines.get(line_index).expect("cannot fetch line");
        let mut char_index = if number.relative_start == 0 { 0 } else { number.relative_start-1 };
        let char_index_stop= if number.relative_end+1 >= lines[line_index].len() { lines[line_index].len()-1 } else { number.relative_end+1 };

        while char_index <= char_index_stop {
            let c = line.chars().nth(char_index).expect("cannot fetch char");
            if !c.is_numeric() && c != '.' && c != '\n' {
                if c == '*' {
                    number.star_adress = Some((char_index, line_index));
                }
                return true;
            }
            char_index+=1;
        }
        line_index+=1;
    }
    false
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }
    
    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("invalid file path");
    //TODO: add parameter to choose operation
    //println!("{}", sum_of_part_numbers(&content));
    println!("{}", get_gear_ratio_sum(&content));
}
