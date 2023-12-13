use std::env;
use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufReader;
use std::io::BufRead;
use num::integer::lcm;

fn main() {
    // let score = part1();
    let score = part2();
    println!("{}", score);
}

fn part2() -> u64{
    // this problem solvable because for NUM(A) == Num(Z)
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut dir: Vec<char> = Vec::new();
    let mut curs: Vec<String> = Vec::new();
    let mut edges: HashMap<String, (String, String)> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if idx == 0 {
            dir = l.trim().chars().collect();
        } else if l.len() > 0 {
            let mut pos = l.find('=').unwrap();
            let root = l[..pos].trim().to_owned();
            pos = l.find(',').unwrap();
            let lc = l[pos-3..pos].trim().to_owned();
            let rc = l[pos+2..pos+5].trim().to_owned();
            edges.insert(root.clone(), (lc.clone(), rc.clone()));
            if root.chars().collect::<Vec<char>>()[2] == 'A' {
                curs.push(root.clone());
            }
        }
    }

    let mut steps_to_z: Vec::<u64> = Vec::new();
    for cur in curs.iter() {
        let mut steps = 0;
        let mut dir_step = 0;
        let mut temp:&String = cur;
        while temp.chars().collect::<Vec<char>>()[2] != 'Z' {
            temp = match dir[dir_step] {
                'L' => &edges[temp].0,
                _ => &edges[temp].1
            };
            dir_step = if dir_step == dir.len() - 1 {0} else {dir_step + 1};
            println!("{}", temp);
            steps += 1;
        }
        steps_to_z.push(steps);
    }
    println!("{:?}", steps_to_z);

    let mut res = steps_to_z[0];
    for i in 1..steps_to_z.len() {
        res = lcm(res, steps_to_z[i]);
    }
    res
}

fn part1() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut dir: Vec<char> = Vec::new();
    let mut edges: HashMap<String, (String, String)> = HashMap::new();

    for (idx, line) in reader.lines().enumerate() {
        let l = line.unwrap();
        if idx == 0 {
            dir = l.trim().chars().collect();
        } else if l.len() > 0 {
            let mut pos = l.find('=').unwrap();
            let root = l[..pos].trim().to_owned();
            pos = l.find(',').unwrap();
            let lc = l[pos-3..pos].trim().to_owned();
            let rc = l[pos+2..pos+5].trim().to_owned();
            edges.insert(root.clone(), (lc.clone(), rc.clone()));
        }
    }
    println!("{:?}", edges);
    let mut cur = "AAA";
    let mut steps = 0;
    let mut dir_step = 0;
    while cur != "ZZZ" {
        cur = match dir[dir_step] {
            'L' => &edges[cur].0,
            _ => &edges[cur].1
        };
        dir_step = if dir_step == dir.len() - 1 {0} else {dir_step + 1};
        println!("{}", cur);
        steps += 1;
    }

    steps
}
