use std::env;
use std::fs;

fn main() {
    // read command line arguments and collect the values into a vector
    // .collect() turns an iterator into a vector
    // needs to annotate the type for .collect because Rust cannot infer collection types
    let args: Vec<String> = env::args().collect();
    // command line arguments: binary name, user args
    // dbg!(args);
    // let query = &args[1];
    // let file_path = &args[2];
    // println!("Searching for {}", query);
    // println!("In file {}", file_path);

    // separate parsing logic into a function
    let config = parse_config(&args);
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    // read files
    // contexts type: std::io::Result<String>
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("With text:\n{contents}")
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> (&str, &str){
    // let query = &args[1];
    // let file_path = &args[2];

    // Instead of referencing args, define Config to contain owned String values
    // args variable in main is the owner of the argument values
    // Config can only borrow, not taking ownership of the argument values
    let query = args[1].clone(); // a full copy of the string values, this is not efficient due to its runtime cost
    let file_path = args[2].clone();

    Config {query, file_path}
}

