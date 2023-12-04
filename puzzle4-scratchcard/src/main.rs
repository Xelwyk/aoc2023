use std::{fs::File, io::Read};
use std::env;

fn calculate_total_points(content: String) -> u32 {
    let map = content.lines()
        .map(|line| line.replace("  ", " ").split(&[':', '|'][..])
            .collect::<Vec<&str>>()
            .iter()
            .filter(|block| !block.trim().contains("Card"))
            .collect::<Vec<&&str>>().iter()
            .map(|nums| nums.trim().split(' ')
                .collect::<Vec<&str>>()
                .iter()
                .map(|num| num.parse::<u32>().expect("cannot parse"))
                .collect::<Vec<u32>>()
            ).collect::<Vec<Vec<u32>>>()
        ).collect::<Vec<Vec<Vec<u32>>>>();
    map.iter().map(|f|
        {
            let exp = (f[1].iter().filter(|p| f[0].contains(p)).collect::<Vec<&u32>>().len()) as u32;
            if exp == 0 { 0 } else { (2 as u32).pow(exp-1) }
        }
    ).sum()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("no file provided");
    }

    let mut content = String::new();
    File::open(&args[1])
        .expect("file not found")
        .read_to_string(&mut content)
        .expect("failed reading file content");

    println!("{}", calculate_total_points(content));
}
