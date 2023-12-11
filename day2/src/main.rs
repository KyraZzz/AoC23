use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::cmp::max;
use std::collections::HashMap;

fn main() {
    let sum = part2();
    println!("{}", sum);
}

fn part2() -> u32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for (idx, line) in reader.lines().enumerate() {
        let mut max_num_color = HashMap::from([
            ("red", 0),
            ("green", 0),
            ("blue", 0)
        ]);
        let s = line.unwrap();
        // split games
        let after_colon_pos = s.find(":").unwrap() + 1;
        let s_trim = &s[after_colon_pos..];
        let games: Vec<&str> = s_trim.split(";").collect();
        for game in games {
            let pairs: Vec<&str> = game.split(",").collect();
            for pair in pairs {
                let pair_trim = pair.trim();
                let res: Vec<&str> = pair_trim.split(" ").collect();
                // check whether it is possible
                let num = res[0].parse::<u32>().unwrap();
                let color = res[1];
                max_num_color.insert(color, max(max_num_color[color], num));
                // println!("num: {}, color:{}, max_num:{}, is_possible: {}", num, color, max_num_color[color], is_possible);
            }
        }
        let mut product = 1;
        for (key, value) in max_num_color {
            product *= value;
        }
        sum += product
    }
    sum
}

fn part1() -> u32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    let max_num_color = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14)
    ]);
    for (idx, line) in reader.lines().enumerate() {
        let mut is_possible = true;
        let s = line.unwrap();
        // split games
        let after_colon_pos = s.find(":").unwrap() + 1;
        let s_trim = &s[after_colon_pos..];
        let games: Vec<&str> = s_trim.split(";").collect();
        for game in games {
            let pairs: Vec<&str> = game.split(",").collect();
            for pair in pairs {
                let pair_trim = pair.trim();
                let res: Vec<&str> = pair_trim.split(" ").collect();
                // check whether it is possible
                let num = res[0].parse::<u32>().unwrap();
                let color = res[1];
                is_possible = max_num_color[color] >= num;
                // println!("num: {}, color:{}, max_num:{}, is_possible: {}", num, color, max_num_color[color], is_possible);
                if !is_possible {
                    break;
                }
            }
            if !is_possible {
                break;
            }
        }
        sum += if is_possible {(idx as u32) + 1} else {0};
    }
    sum
}
