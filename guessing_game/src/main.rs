use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Enter a number.");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("No number available.");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Guess is too small."),
            Ordering::Equal => {
                println!("Correct.");
                break;
            }
            Ordering::Greater => println!("Guess is too big."),
        }
        println!("Your guess: {guess}");
    }
    println!("The secret num is: {secret_num}")
}
