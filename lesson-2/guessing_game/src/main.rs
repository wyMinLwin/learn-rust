use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let sceret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Enter your guess!");
    
        let mut guess = String::new();
    
    
        io::stdin().read_line(&mut guess).expect("Failed To Read Line");
    
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&sceret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}