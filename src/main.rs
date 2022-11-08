use std::io;
use rayon::prelude::*;
use std::fs;

pub fn get_user_input(prompt: &str) -> i32 {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let number: i32 = user_input.trim().parse().expect("Please enter an integer!");

    return number;
}

fn primes_up_to(number: i32) -> Vec<i32> {
    (2..=number).into_par_iter().filter(is_prime).collect::<Vec<i32>>()
}

fn is_prime(number: &i32) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let number_dereferenced: i32 = *number;
    let root_of_number: i32 = (number_dereferenced as f64).sqrt() as i32;
    !(3..root_of_number).step_by(2).any(|n| root_of_number % n == 0)
}

fn main() {
    let user_input: i32 = get_user_input("Enter a positive integer greater than 2: ");
    
    println!("Calculating primes up to {}...", user_input);
    let primes: Vec<i32> = primes_up_to(user_input);
    
    let prime_str: String = format!("{:?}", primes);
    println!("Saving primes to file...");
    fs::write("primes.txt", prime_str).expect("Unable to write file.");
    println!("Primes saved to file.");
}