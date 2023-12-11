use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // let sum = part1();
    let sum = part2();
    println!("{}", sum);
}

fn part2() -> u32 {
    let mut sum = 0;
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let arr: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().split("")
        .map(|c| c.parse::<char>().unwrap_or_default())
        .collect())
        .collect();

    let mut special_pos: HashSet<(usize, usize)> = HashSet::new();
    // get all positions of special character *
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if arr[i][j] == '*' {
                special_pos.insert((i,j));
            }
        }
    }
    let mut adjacent_num: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    for pos in special_pos.iter() {
        adjacent_num.insert(*pos, Vec::new());
    }
    // find numbers where exactly two part numbers are adjacent to *
    // get all pos that adjacent to a special character
    let dx: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    for i in 0..arr.len() {
        let mut cur: Vec<u32> = Vec::new();
        let mut cur_set: HashSet<(usize, usize)> = HashSet::new();
        for j in 0..arr[0].len() {
            if arr[i][j].is_numeric() {
                cur.push(arr[i][j].to_digit(10).unwrap());
                // check whether surrounded by a special character
                for d in 0..dx.len() {
                    let nx = (i as i32) + dx[d];
                    let ny = (j as i32) + dy[d];
                    if (nx < 0) | (nx >= (arr.len() as i32)) | (ny < 0) | (ny >= (arr[0].len() as i32)) {
                        continue;
                    }
                    if special_pos.contains(&(nx as usize, ny as usize)) {
                        cur_set.insert((nx as usize, ny as usize));
                    }
                }
            }
            // add numbers that are valid
            if (j == arr[i].len() - 1) | (!arr[i][j].is_numeric()) {
                if !cur_set.is_empty() {
                    let mut temp: u32 = 0;
                    let mut digit_power: u32 = 0;
                    let base:u32 = 10;
                    while !cur.is_empty() {
                        temp += cur.pop().unwrap() * base.pow(digit_power);
                        digit_power += 1;
                    }
                    assert_eq!(cur.len(), 0);
                    for k in &cur_set {
                        adjacent_num.entry(*k).or_insert_with(Vec::new).push(temp);
                        continue;
                    }
                }
                cur.clear();
                cur_set.clear();
            }
        }
    }

    for (_k, v) in adjacent_num.iter() {
        if v.len() == 2 {
            let prod_v = v.iter().fold(1, |mut prod, &val| {prod *= val; prod});
            sum += prod_v;
        }
    }

    sum
}

fn part1() -> u32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let arr: Vec<Vec<char>> = reader.lines()
        .map(|l| l.unwrap().split("")
        .map(|c| c.parse::<char>().unwrap_or_default())
        .collect())
        .collect();

    let mut special_pos: HashSet<(usize, usize)> = HashSet::new();
    // get all positions of special characters
    for i in 0..arr.len() {
        for j in 0..arr[0].len() {
            if (arr[i][j].is_ascii()) & (arr[i][j] != '.') & (arr[i][j] != '\0') & (!arr[i][j].is_numeric()) {
                special_pos.insert((i,j));
            }
        }
    }

    let mut sum = 0;
    // get all pos that adjacent to a special character
    let dx: [i32; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    let dy: [i32; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    for i in 0..arr.len() {
        let mut cur: Vec<u32> = Vec::new();
        let mut flag: bool = false;
        for j in 0..arr[0].len() {
            if arr[i][j].is_numeric() {
                cur.push(arr[i][j].to_digit(10).unwrap());
                // check whether surrounded by a special character
                if !flag {
                    for d in 0..dx.len() {
                        let nx = (i as i32) + dx[d];
                        let ny = (j as i32) + dy[d];
                        if (nx < 0) | (nx >= (arr.len() as i32)) | (ny < 0) | (ny >= (arr[0].len() as i32)) {
                            continue;
                        }
                        if special_pos.contains(&(nx as usize, ny as usize)) {
                            flag = true;
                        }
                    }
                }
            }
            // add numbers that are valid
            if (j == arr[i].len() - 1) | (!arr[i][j].is_numeric()) {
                if flag {
                    let mut temp: u32 = 0;
                    let mut digit_power: u32 = 0;
                    let base:u32 = 10;
                    while !cur.is_empty() {
                        temp += cur.pop().unwrap() * base.pow(digit_power);
                        digit_power += 1;
                    }
                    assert_eq!(cur.len(), 0);
                    sum += temp;
                }
                cur.clear();
                flag = false;
            }
        }
    }
    sum
}
