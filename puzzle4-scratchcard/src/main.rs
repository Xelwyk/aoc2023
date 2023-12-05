use std::{fs::File, io::Read};
use std::{env, vec};

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

fn calculate_total_scratchcards(content: String) -> u32 {
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

    let mut ubermap: Vec<Vec<Vec<Vec<u32>>>> = vec![];
    for ele in map {
        let mut wrap:Vec<Vec<Vec<u32>>> = vec![];
        wrap.push(ele);
        ubermap.push(wrap);
    }

    let mut line = 0;
    while line < ubermap.len() {
        let mut instance = 0;
        while instance < ubermap[line].len() {   
            println!("line:{} ins:{} - insts:{}",line,instance, ubermap[line].len());
            let mut count = ubermap[line][instance][1].iter().filter(|num| ubermap[line][instance][0].contains(num)).collect::<Vec<&u32>>().len();
            let mut j = 1;
            while count > 0 {
                //println!("count:{}",count);
                let clone = ubermap[line+j][0].clone();
                ubermap[line+j].push(clone);
                count -= 1;
                j += 1;
            }
            instance += 1;
        }
        line += 1;
    }
    for ele in &ubermap {
        println!("{} - {:?}", ele.len(),ele[0]);
        println!("------------------------");
    }
    return ubermap.iter().map(|e| e.len() as u32).sum()
    //return 0
    
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

    //TODO: read oper from cmd arg
    //println!("{}", calculate_total_points(content));
    println!("{}", calculate_total_scratchcards(content));

}
