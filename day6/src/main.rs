use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() {
    // let prod = part1();
    let prod = part2();
    println!("{}", prod);
}

fn part2() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut time: u64 = 0;
    let mut distance: u64 = 0;

    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let colon_pos:Option<usize> = l.find(':');
        let pattern = Regex::new(r" +").unwrap();
        let base:u64 = 10;
        if idx == 0 {
            let temp:String = l[colon_pos.unwrap()+1..].split_whitespace().collect();
            time = temp.parse::<u64>().unwrap();
        } else {
            let temp:String = l[colon_pos.unwrap()+1..].split_whitespace().collect();
            distance = temp.parse::<u64>().unwrap();
        }
    }
    let mut p1 = time / 2;
    let mut p2 = if time % 2 == 1 {p1 + 1} else {p1};
    let mut cnt = 0;
    while (p1 * p2 > distance) & (p1 >= 0) & (p2 <= time) {
        cnt += if p1 != p2 {2} else {1};
        p1 -= 1;
        p2 += 1;
    }
    cnt
}

fn part1() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut times: Vec<u64> = Vec::new();
    let mut distances: Vec<u64> = Vec::new();

    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let colon_pos:Option<usize> = l.find(':');
        let pattern = Regex::new(r" +").unwrap();
        if idx == 0 {
            let temp:Vec<&str> = pattern.split(l[colon_pos.unwrap()+1..].trim()).collect();
            times = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        } else {
            let temp:Vec<&str> = pattern.split(l[colon_pos.unwrap()+1..].trim()).collect();
            distances = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
        }
    }
    let mut prod = 1;
    for i in 0..times.len() {
        let time = times[i];
        let distance = distances[i];
        let mut p1 = time / 2;
        let mut p2 = if time % 2 == 1 {p1 + 1} else {p1};
        let mut cnt = 0;
        while (p1 * p2 > distance) & (p1 >= 0) & (p2 <= time) {
            cnt += if p1 != p2 {2} else {1};
            p1 -= 1;
            p2 += 1;
        }
        prod *= cnt;
    }

    prod
}