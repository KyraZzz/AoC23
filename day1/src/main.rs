use std::env;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    // let sum = part1();
    let sum = part2();

    println!("{}", sum);
}

fn _convert_to_digit(s: &str) -> (u32, u32){
    let mut res = 0;
    let mut len = 0;
    if s.len() >= 3 {
        res = match &s[..3] {
            "one" => 1,
            "two" => 2,
            "six" => 6,
            _ => 0,
        };
        len = 3;
    }
    if (res == 0) & (s.len() >= 4) {
        res = match &s[..4] {
            "four" => 4,
            "five" => 5,
            "nine" => 9,
            _ => 0,
        };
        len = 4;
    }
    if (res == 0) & (s.len() >= 5) {
        res = match &s[..5] {
            "three" => 3,
            "seven" => 7,
            "eight" => 8,
            _ => 0
        };
        len = 5;
    }
    (res, len)
}

fn part2() -> u32{
    let args: Vec<String> = env::args().collect();
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let mut digits: Vec<u32> = Vec::new();
        let s: String = line.unwrap();
        let char_vec: Vec<char> = s.chars().collect();
        let mut idx = 0;
        while idx < s.len() {
            if char_vec[idx].is_numeric() {
                digits.push(char_vec[idx].to_digit(10).unwrap());
            } else {
                let (res, length) = _convert_to_digit(&s[idx..]);
                if res != 0 {
                    // idx += (length - 1) as usize;
                    digits.push(res);
                }
            }
            idx += 1;
            
        }
        let sum_line = if digits.len() > 0 {digits[0] * 10 + digits[digits.len() - 1]} else {0};
        sum += sum_line;
    }
    sum   
}

fn part1() -> u32{
    // String data type is mutable, and is different from immutable string literals (i.e., hard-coded strings)
    // String allocated on the heap, can store an amount of text that is unknown to us at compile time
    let args: Vec<String> = env::args().collect();
    // println!("{}", args[1]);
    // Rust move: make a shallow copy and invalidates the first variable
    // A type with Copy trait will not move but deeply copied
    // Reference allow to refer to some value without taking ownership of it
    // Normal references are immutable
    let file = fs::File::open(&args[1]).expect("File not found");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        // when you have an operation that either return a T or fail, it gives a type Result<T,E> or Option<T>
        // unwrap(self) -> T returns embedded T if it has one else panic
        // best used when positively sure you don't have an error
        let char_vec: Vec<char> = line.unwrap().chars().collect();
        let mut p1: usize = 0;
        let mut p2: usize = char_vec.len() - 1;
        let mut found_first: bool = false;
        let mut found_last: bool = false;
        
        while (!found_first) & (p1 < char_vec.len()) {
            let c1: char = char_vec[p1];
            if c1.is_numeric(){
                found_first = true;
                sum += c1.to_digit(10).unwrap() * 10;
            } else {
                p1 += 1;
            }
        }
        while (!found_last) & (p2 >= 0) {
            let c2: char = char_vec[p2];
            if c2.is_numeric(){
                found_last = true;
                sum += c2.to_digit(10).unwrap();
            } else {
                p2 -= 1;
            }
        }
        if !(found_first & found_last){
            println!("Not found");
        }
    }
    sum
}
