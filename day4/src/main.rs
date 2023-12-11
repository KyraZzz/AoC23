use std::env;
use std::fs;
use std::collections::HashSet;
use std::collections::HashMap;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;

fn main() {
    // let sum = part1();
    let sum = part2();
    println!("{}", sum);
}

fn part2() -> u32 {
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    let mut counts: HashMap<u32, u32> = HashMap::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();

        let colon_pos = l.find(":").unwrap();
        let parts: Vec<&str> = l[colon_pos+1..].split('|').collect();
        let pattern = Regex::new(r" +").unwrap();
        let win_seq: HashSet<&str> = pattern.split(parts[0].trim()).collect();
        let our_seq: Vec<&str> = pattern.split(parts[1].trim()).collect();
        let mut contained: Vec<&str> = Vec::new();

        let mut count_l = 0;
        for n in our_seq {
            if win_seq.contains(n) {
                contained.push(n);
                count_l += 1;
            }
        }

        counts.entry((i+1) as u32).and_modify(|x| {*x+=1}).or_insert(1);
        let value_l = counts.get(&((i+1) as u32)).unwrap().clone();
        for j in i..(i+count_l) {
            counts.entry((j+2) as u32).and_modify(|x| {*x+=value_l}).or_insert(value_l);
        }
    }
    for (_k, v) in counts.iter() {
        sum += v;
    }

    sum
}

fn part1() -> u32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    for line in reader.lines() {
        let l = line.unwrap();

        let colon_pos = l.find(":").unwrap();
        let parts: Vec<&str> = l[colon_pos+1..].split('|').collect();
        let pattern = Regex::new(r" +").unwrap();
        let win_seq: HashSet<&str> = pattern.split(parts[0].trim()).collect();
        let our_seq: Vec<&str> = pattern.split(parts[1].trim()).collect();
        let mut contained: Vec<&str> = Vec::new();

        let mut count_l = 0;
        for n in our_seq {
            if win_seq.contains(n) {
                contained.push(n);
                count_l += 1;
            }
        }

        let base: u32 = 2;
        sum += if count_l > 0 {base.pow(count_l - 1)} else {0};
    }

    sum
}
