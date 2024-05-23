use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashSet;

fn main() {
    let score = part1();
    // let score = part2();
    println!("{}", score);
}

fn part1() -> i32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut clean_rows: HashSet<usize> = HashSet::new();
    let mut clean_columns: HashSet<usize> = HashSet::new();
    let mut gals: HashSet<(usize, usize)> = HashSet::new();
    for (i, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if i == 0 {
            clean_columns = HashSet::from_iter((0..l.len()).collect::<Vec<usize>>());
        }
        let pos: Vec<_> = l.match_indices("#").collect();
        if pos.len() == 0 {
            clean_rows.insert(i);
        }
        for p in pos.iter() {
            clean_columns.remove(&(p.0)); 
            gals.insert((i, p.0));
        }
    }
    // println!("{:?}", clean_columns);
    // println!("{:?}", clean_rows);
    // println!("{:?}", gals);

    let mut gals_expand: Vec<(i32, i32)> = Vec::new();
    for gal in gals.iter() {
        let mut ni = (gal.0).clone();
        let mut nj = (gal.1).clone();
        for row in clean_rows.iter() {
            if *row < gal.0 {
                ni += 1;
            }
        }
        for col in clean_columns.iter() {
            if *col < gal.1 {
                nj += 1;
            }
        }
        gals_expand.push((ni as i32, nj as i32));
    }
    // println!("{:?}", gals_expand);
    let mut sum = 0;
    for i in 0..gals_expand.len() {
        for j in (i+1)..gals_expand.len() {
            let p = &gals_expand[i];
            let q = &gals_expand[j];
            sum += (p.0 - q.0).abs() + (p.1 - q.1).abs();
            // println!("{:?} {:?} {}", p, q, ((p.0 - q.0).abs() + (p.1 - q.1).abs())); 
        }
    }

    sum
}