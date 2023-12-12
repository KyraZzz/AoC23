use std::env;
use std::fs;
use std::collections::HashMap;
use itertools::Itertools;
use std::io::BufReader;
use std::io::BufRead;
use regex::Regex;
use std::cmp::Ordering;

fn main() {
    // let score = part1();
    let score = part2();
    println!("{}", score);
}

fn part2() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut hands: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    for i in 0..7 {
        hands.insert(i, Vec::new());
    }

    fn _compare(a_: &(String, u64), b_: &(String, u64)) -> Ordering {
        let tuples: Vec<char> = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
        let mut rank_map:HashMap<char, usize> = HashMap::new();
        for i in 0..tuples.len() {
            rank_map.insert(tuples[i], i);
        }
        let mut p1 = 0;
        let mut p2 = 0;
        let a = (a_.0).chars().collect::<Vec<char>>();
        let b = (b_.0).chars().collect::<Vec<char>>();
        while (p1 < a.len()) & (p2 < b.len()) {
            let ap = a[p1];
            let bp = b[p2];
            if rank_map[&ap] == rank_map[&bp] {
                p1 += 1;
                p2 += 1;
            } else if rank_map[&ap] < rank_map[&bp] {
                // a is greater
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        // if two strings are exactly the same, return a > b
        return Ordering::Greater;
    }

    for line in reader.lines() {
        let pattern = Regex::new(r" +").unwrap();
        let l = line.unwrap();
        let temp:Vec<&str> = pattern.split(l.trim()).collect();
        let hand:String = temp[0].to_owned();
        let bid:u64 = temp[1].parse::<u64>().unwrap();

        // which type of hand?
        let mut char_map: HashMap<char, u64> = HashMap::new();
        let mut num_j = 0;
        for c in hand.chars() {
            if c == 'J' {
                num_j += 1;
                continue;
            }
            char_map.entry(c).and_modify(|t| {*t+=1}).or_insert(1);
        }
        // wild card joker
        let mut occur: Vec<u64> = vec![0;5];
        for (i, v) in char_map.values().sorted().rev().enumerate() {
            if i == 0 {
                occur[(*v - 1 + num_j) as usize] += 1;
                num_j = 0;
            } else {
                occur[(*v - 1) as usize] += 1;
            }
        }
        if num_j != 0 {
            // all joker cards
            occur[4] = 1;
        }
        let t = match *occur {
            [_, _, _, _, 1] => 6,
            [_, _, _, 1, 0] => 5,
            [_, 1, 1, 0, 0] => 4,
            [_, 0, 1, 0, 0] => 3,
            [_, 2, 0, 0, 0] => 2,
            [_, 1, 0, 0, 0] => 1,
            _ => 0,
        };
        hands.entry(t).and_modify(|v| v.push((hand, bid)));
    }
    // for each type, sort the hands
    // println!("{:?}", hands);
    let mut rank = 1;
    let mut score = 0;
    for key in hands.keys().sorted() {
        // println!("{}: {:?}", key, hands[key]);
        if hands[key].len() > 0 {
            // sort the vec
            let mut v = hands[key].clone();
            v.sort_unstable_by(_compare);
            for a in v.iter() {
                score += a.1 * rank;
                rank += 1;
            }
            // println!("{:?}", v);
        }
    }

    score
}

fn part1() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut hands: HashMap<u64, Vec<(String, u64)>> = HashMap::new();
    for i in 0..7 {
        hands.insert(i, Vec::new());
    }

    fn _compare(a_: &(String, u64), b_: &(String, u64)) -> Ordering {
        let tuples: Vec<char> = vec!['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
        let mut rank_map:HashMap<char, usize> = HashMap::new();
        for i in 0..tuples.len() {
            rank_map.insert(tuples[i], i);
        }
        let mut p1 = 0;
        let mut p2 = 0;
        let a = (a_.0).chars().collect::<Vec<char>>();
        let b = (b_.0).chars().collect::<Vec<char>>();
        while (p1 < a.len()) & (p2 < b.len()) {
            let ap = a[p1];
            let bp = b[p2];
            if rank_map[&ap] == rank_map[&bp] {
                p1 += 1;
                p2 += 1;
            } else if rank_map[&ap] < rank_map[&bp] {
                // a is greater
                return Ordering::Greater;
            } else {
                return Ordering::Less;
            }
        }
        // if two strings are exactly the same, return a > b
        return Ordering::Greater;
    }

    for line in reader.lines() {
        let pattern = Regex::new(r" +").unwrap();
        let l = line.unwrap();
        let temp:Vec<&str> = pattern.split(l.trim()).collect();
        let hand:String = temp[0].to_owned();
        let bid:u64 = temp[1].parse::<u64>().unwrap();

        // which type of hand?
        let mut char_map: HashMap<char, u64> = HashMap::new();
        for c in hand.chars() {
            char_map.entry(c).and_modify(|t| {*t+=1}).or_insert(1);
        }
        let mut occur: Vec<u64> = vec![0;5];
        for v in char_map.values() {
            occur[(*v - 1) as usize] += 1;
        }
        let t = match *occur {
            [_, _, _, _, 1] => 6,
            [_, _, _, 1, 0] => 5,
            [_, 1, 1, 0, 0] => 4,
            [_, 0, 1, 0, 0] => 3,
            [_, 2, 0, 0, 0] => 2,
            [_, 1, 0, 0, 0] => 1,
            _ => 0,
        };
        hands.entry(t).and_modify(|v| v.push((hand, bid)));
    }
    // for each type, sort the hands
    // println!("{:?}", hands);
    let mut rank = 1;
    let mut score = 0;
    for key in hands.keys().sorted() {
        // println!("{}: {:?}", key, hands[key]);
        if hands[key].len() > 0 {
            // sort the vec
            let mut v = hands[key].clone();
            v.sort_unstable_by(_compare);
            for a in v.iter() {
                score += a.1 * rank;
                rank += 1;
            }
            // println!("{:?}", v);
        }
    }

    score
}
