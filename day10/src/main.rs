use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::max;

fn main() {
    // let score = part1();
    let score = part2();
    println!("{}", score);
}

fn part2() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut mapping:HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut start_pos: (i64, i64) = (0, 0);
    let tiles: String = String::from("|-LJ7F");
    let dx1: [i64; 6] = [-1, 0, -1, -1, 0, 0];
    let dy1: [i64; 6] = [0, -1, 0, 0, -1, 1];
    let dx2: [i64; 6] = [1, 0, 0, 0, 1, 1];
    let dy2: [i64; 6] = [0, 1, 1, -1, 0, 0];
    let mut width = 0;
    let mut height = 0;

    fn _is_valid(x: usize, y: usize, width: usize, height: usize) -> bool {
        x >= 0 && x < height && y >= 0 && y < width
    }

    for (i, line) in reader.lines().enumerate() {
        let l:Vec<char> = line.unwrap().chars().collect();
        width = 0;
        for (j, c) in l.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i as i64, j as i64);
            } else if *c != '.' {
                let idx = tiles.find(*c).unwrap();
                let n1 = (i as i64 + dx1[idx], j as i64 + dy1[idx]);
                let n2 = (i as i64 + dx2[idx], j as i64 + dy2[idx]);
                mapping.insert((i as i64, j as i64), vec![n1, n2]);
            }
            width += 1;
        }
        height += 1;
    }
    // println!("width {} height {}", width, height);

    let dx: [i64; 4] = [-1, 1, 0, 0];
    let dy: [i64; 4] = [0, 0, -1, 1];
    let mut neighbors: Vec<(i64, i64)> = Vec::new();
    for i in 0..dx.len() {
        let x = start_pos.0 + dx[i];
        let y = start_pos.1 + dy[i];
        if _is_valid(x as usize, y as usize, width, height) & mapping.contains_key(&(x,y)) {
            for v in &mapping[&(x, y)] {
                if neighbors.len() == 2 {
                    break;
                }
                if *v == start_pos {
                    neighbors.push((x, y));
                }
            }
        }
        mapping.insert(start_pos, neighbors.clone());
    }
    // println!("{:?}", mapping);

    let mut visited:HashSet<(i64, i64)> = HashSet::new();
    fn dfs(width: usize, height: usize, start_pos: (i64, i64), visited: &mut HashSet<(i64, i64)>, mapping: &mut HashMap<(i64, i64), Vec<(i64, i64)>>) {
        let mut stack:Vec<(i64, i64)> = Vec::new();
        stack.push(start_pos);
        while !stack.is_empty() {
            let cur = stack.pop().unwrap();
            visited.insert(cur);
            for v in &mapping[&cur] {
                if !visited.contains(v) & _is_valid(v.0 as usize, v.1 as usize, width, height) {
                    stack.push(*v);
                }
            }
        }
    }

    dfs(width, height, start_pos, &mut visited, &mut mapping);
    println!("visited: {:?}", visited.len());

    fn dfs2(width: usize, height: usize, start_pos: (i64, i64), visited: &mut HashSet<(i64, i64)>, visited2: &mut HashSet<(i64, i64)>, mapping: &mut HashMap<(i64, i64), Vec<(i64, i64)>>) {
        let mut stack:Vec<(i64, i64)> = Vec::new();
        stack.push(start_pos);
        visited2.insert(start_pos);
        while !stack.is_empty() {
            println!("{:?}", stack);
            let cur = stack.pop().unwrap();
            let dx: [i64; 4] = [-1, 1, 0, 0];
            let dy: [i64; 4] = [0, 0, -1, 1];
            for idx in 0..dx.len() {
                let x = cur.0 + dx[idx];
                let y = cur.1 + dy[idx];
                if _is_valid(x as usize, y as usize, width, height) & !visited.contains(&(x, y)) & !visited2.contains(&(x, y)) {
                    visited2.insert((x, y));
                    stack.push((x, y))
                }
            }
        }
    }

    // flood
    let mut visited2:HashSet<(i64, i64)> = HashSet::new();
    for i in 0..height {
        let js = if (i == 0) | (i == height - 1) {(0..width).collect::<Vec<_>>()} else {Vec::from([0, width-1])};
        for j in js {
            if !visited.contains(&(i as i64, j as i64)) & !visited2.contains(&(i as i64, j as i64)) & _is_valid(i, j, width, height) {
                dfs2(width, height, (i as i64, j as i64), &mut visited, &mut visited2, &mut mapping);
                println!("{:?}", visited2.len());
            }
        }
    }
    println!("{} {} {}",height * width, visited.len(), visited2.len());

    (height * width - visited.len() - visited2.len()) as u64
}

fn part1() -> u64{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut mapping:HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut start_pos: (i64, i64) = (0, 0);
    let tiles: String = String::from("|-LJ7F");
    let dx1: [i64; 6] = [-1, 0, -1, -1, 0, 0];
    let dy1: [i64; 6] = [0, -1, 0, 0, -1, 1];
    let dx2: [i64; 6] = [1, 0, 0, 0, 1, 1];
    let dy2: [i64; 6] = [0, 1, 1, -1, 0, 0];
    let mut width = 0;
    let mut height = 0;

    fn _is_valid(x: usize, y: usize, width: usize, height: usize) -> bool {
        x >= 0 && x < width && y >= 0 && y < height
    }

    for (i, line) in reader.lines().enumerate() {
        let l:Vec<char> = line.unwrap().chars().collect();
        width = 0;
        for (j, c) in l.iter().enumerate() {
            if *c == 'S' {
                start_pos = (i as i64, j as i64);
            } else if *c != '.' {
                let idx = tiles.find(*c).unwrap();
                let n1 = (i as i64 + dx1[idx], j as i64 + dy1[idx]);
                let n2 = (i as i64 + dx2[idx], j as i64 + dy2[idx]);
                mapping.insert((i as i64, j as i64), vec![n1, n2]);
            }
            width += 1;
        }
        height += 1;
    }
    // println!("{:?}", start_pos);

    let dx: [i64; 4] = [-1, 1, 0, 0];
    let dy: [i64; 4] = [0, 0, -1, 1];
    let mut neighbors: Vec<(i64, i64)> = Vec::new();
    for i in 0..dx.len() {
        let x = start_pos.0 + dx[i];
        let y = start_pos.1 + dy[i];
        if _is_valid(x as usize, y as usize, width, height) & mapping.contains_key(&(x,y)) {
            for v in &mapping[&(x, y)] {
                if neighbors.len() == 2 {
                    break;
                }
                if *v == start_pos {
                    neighbors.push((x, y));
                }
            }
        }
        mapping.insert(start_pos, neighbors.clone());
    }
    // println!("{:?}", mapping);

    let mut queue:Vec<((i64, i64), u64)> = Vec::new();
    queue.push((start_pos, 0));
    let mut visited:HashSet<(i64, i64)> = HashSet::new();
    let mut max_dist = 0;
    while !queue.is_empty() {
        // println!("{:?}", queue);
        let cur = queue.remove(0);
        visited.insert(cur.0);
        max_dist = max(max_dist, cur.1);
        for v in &mapping[&(cur.0)] {
            if !visited.contains(v) {
                queue.push((*v, (cur.1) + 1));
            }
        }
    }

    max_dist
}