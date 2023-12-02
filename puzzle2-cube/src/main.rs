use std::env;
use std::fs;
use std::collections::HashMap;

fn get_id_sum(content: &String, target_subset: &HashMap<&str, u32>) -> u32 {
    let mut game_id: u32;
    let mut game_sum = 0;
    for line in content.lines() {
        game_id = get_id(line);
        if is_possible(line, target_subset) {
            game_sum += game_id;
        }
    }
    game_sum
}

fn get_id(line: &str) -> u32 {
    let game_id_parts = line.split(&[' ',':'][..]).collect::<Vec<&str>>();
    if game_id_parts.len() < 2 {
        panic!("game ID parts are incorrect");
    }
    game_id_parts[1].parse().expect("parsing of game ID failed")
}

fn is_possible(line: &str, target_subset: &HashMap<&str, u32>) -> bool {
    let subsets = get_subsets(line);
    for subset in subsets {
        if !subset_supported(&subset, target_subset) {
            return false
        }
    }
    true
}

fn get_subsets(line: &str) -> Vec<HashMap<&str, u32>> {
    let mut parsed_subsets: Vec<HashMap<&str, u32>> = vec![];
    let mut line_subsets: Vec<&str> = line.split(':').collect();

    if line_subsets.len() < 2 {
        panic!("line is missing subset");
    }

    line_subsets = line_subsets[1].split(';').collect();

    for subset in line_subsets {
        let mut parsed_subset = HashMap::new();
        let cubes: Vec<&str> = subset.split(',').collect();
        for cube in cubes {
            let cube_info: Vec<&str> = cube.trim().split(' ').collect();
            if cube_info.len() < 2 {
                panic!("cube info is incorrect");
            }
            let color = cube_info[1];
            let value = cube_info[0].parse().expect("failed parsing cube number");
            parsed_subset.insert(color, value);
        }
        parsed_subsets.push(parsed_subset);
    }
    parsed_subsets
}

fn subset_supported(subset: &HashMap<&str, u32>, target_subset: &HashMap<&str, u32>) -> bool {
    for (color, value) in subset {
        let target_value = target_subset.get(color).expect("unsupported color in target_subset");
        if target_value < value {
            return false;
        }
    }
    true
}


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 5 {
        panic!("program is missing arguments");
    }

    let file_path = &args[1];
    let content = fs::read_to_string(file_path).expect("invalid file path");

    let mut target_subset: HashMap<&str, u32> = HashMap::new();
    target_subset.insert("red", args[2].parse().expect("failed parsing of red parameter"));
    target_subset.insert("green", args[3].parse().expect("failed parsing of green parameter"));
    target_subset.insert("blue", args[4].parse().expect("failed parsing of blue parameter"));
    let target_subset = target_subset;

    println!("{}", get_id_sum(&content, &target_subset));
}
