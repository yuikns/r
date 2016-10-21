extern crate r;
extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;
use r::math;

fn main() {
    println!("Input Number:");
    let mut guess = String::new();
    
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    
    println!("input: {}", guess);
    
    let random_number: i64 = rand::thread_rng().gen_range(-60, 0);
    
    println!("random number: {}", random_number);
    
    let guess_number: i64 = guess.trim().parse().expect("Incorrect input format, not a number");
    
    match guess_number.cmp(&math::abs(random_number)) {
        Ordering::Less => println!("guess {} is less than random {}", guess, random_number),
        Ordering::Greater => println!("guess {} is greater than random {}", guess, random_number),
        Ordering::Equal => println!("guess {} equals random {}", guess, random_number),
    }

    println!("hello world!");
}
