use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use core::cmp::min;
use regex::Regex;

fn main() {
    // let min = part1();
    let min = part2();
    println!("{}", min);
}

fn part2() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut mapping: HashMap<u64, Vec<(u64, u64, u64)>> = HashMap::new();
    for i in 1..8{
        mapping.insert(i, Vec::new());
    }
    let mut cnt:u64 = 0;
    let mut seeds:Vec<(u64, u64)> = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let colon_pos:Option<usize> = l.find(':');
        let pattern = Regex::new(r" +").unwrap();
        if idx == 0 {
            let temp:Vec<&str> = pattern.split(l[colon_pos.unwrap()+1..].trim()).collect();
            let temp2:Vec<u64> = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
            for i in 0..(temp2.len()/2) {
                let s = temp2[2*i];
                let r = temp2[2*i+1];
                seeds.push((s, s+r-1));
            }
        } else if !colon_pos.is_none() {
            cnt += 1;
        } else if l.len() > 0 {
            let temp: Vec<&str> = pattern.split(l.trim()).collect();
            let range: Vec<u64> = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
            let source = range[1];
            let dest = range[0];
            let step = range[2];
            mapping.entry(cnt).and_modify(|h| {h.push((source, source+step-1, dest));});
        }
    }

    for k in 1..8 {
        let mut v:Vec<(u64, u64, u64)> = mapping.get(&k).unwrap().clone();
        v.sort_by_key(|k| k.0);
        let mut temp: Vec<(u64, u64)> = Vec::new();
        for seed in seeds {
            let mut s = seed.0;
            let e = seed.1;
            for (s2, e2, d) in v.iter() {
                while (s <= *e2) & (s < e){
                    if s < *s2 {
                        let end = min(e, *s2 - 1);
                        temp.push((s, end));
                        s = end + 1;
                    } else {
                        let end = min(e, *e2);
                        temp.push((s - *s2 + *d, end - *s2 + *d));
                        s = end + 1;
                    }
                }
            }
            if s < e {
                temp.push((s, e));
            }   
        }
        seeds = temp.clone();
    }

    seeds.sort_by_key(|k| k.0);
    seeds[0].0
}

fn part1() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    
    let mut mapping: HashMap<u64, HashSet<Vec<u64>>> = HashMap::new();
    for i in 1..8{
        mapping.insert(i, HashSet::new());
    }
    let mut cnt:u64 = 0;
    let mut seeds:Vec<u64> = Vec::new();
    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        let colon_pos:Option<usize> = l.find(':');
        let pattern = Regex::new(r" +").unwrap();
        if idx == 0 {
            let temp:Vec<&str> = pattern.split(l[colon_pos.unwrap()+1..].trim()).collect();
            seeds = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
            // println!("{:?}", seeds);
        } else if !colon_pos.is_none() {
            cnt += 1;
        } else if l.len() > 0 {
            let temp: Vec<&str> = pattern.split(l.trim()).collect();
            let range: Vec<u64> = temp.into_iter().map(|s| s.parse::<u64>().unwrap()).collect();
            mapping.entry(cnt).and_modify(|h| {h.insert(range);});
        }
    }

    let mut locs: Vec<u64> = Vec::new();
    for seed in seeds {
        // println!("seed: {}", seed);
        let mut next_key = seed;
        for k in 1..8 {
            let v = mapping.get(&k).unwrap();
            for arr in v.iter() {
                if (next_key >= arr[1]) & (next_key < arr[1] + arr[2]) {
                    next_key = arr[0] + (next_key - arr[1]);
                    break;
                }
            }
            // println!("next_key: {}, cnt: {}", next_key, k);
        }
        locs.push(next_key);
    }

    *locs.iter().min().unwrap()
}
