use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() {
    // let score = part1();
    let score = part2();
    println!("{}", score);
}

fn part2() -> i64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let pattern = Regex::new(r" +").unwrap();
        let temp:Vec<&str> = pattern.split(l.trim()).collect();
        let mut seq:Vec<i64> = temp.iter().map(|c| c.parse::<i64>().unwrap()).collect();
        let mut pred = seq[0];
        let mut alt_op = -1;
        // println!("{:?}", seq);
        while seq.iter().any(|c| *c != 0) {
            seq = seq.windows(2).map(|w| w[1] - w[0]).collect();
            pred += seq[0] * alt_op;
            alt_op *= -1;
            // println!("{:?}", seq);
        }
        sum += pred;
    }

    sum
}

fn part1() -> i64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        let pattern = Regex::new(r" +").unwrap();
        let temp:Vec<&str> = pattern.split(l.trim()).collect();
        let mut seq:Vec<i64> = temp.iter().map(|c| c.parse::<i64>().unwrap()).collect();
        let mut pred = 0;
        // println!("{:?}", seq);
        while seq.iter().any(|c| *c != 0) {
            pred += seq[seq.len()-1];
            seq = seq.windows(2).map(|w| w[1] - w[0]).collect();
            // println!("{:?}", seq);
        }
        sum += pred;
    }

    sum
}
