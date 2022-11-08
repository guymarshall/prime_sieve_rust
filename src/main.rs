use std::io;
use rayon::prelude::*;

pub fn get_user_input(prompt: &str) -> i128 {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let number: i128 = user_input.trim().parse().expect("Please enter an integer!");

    return number;
}

fn primes_up_to(number: i128) -> Vec<i128> {
    (2..=number).into_par_iter().filter(is_prime).collect::<Vec<i128>>()
}

fn is_prime(number: &i128) -> bool {
    if number % 2 == 0 {
        return false;
    }
    let number_dereferenced: i128 = *number;
    let root_of_number: i128 = (number_dereferenced as f64).sqrt() as i128;
    !(3..root_of_number).step_by(2).any(|n| number % n == 0)
}

fn main() {
    let user_input: i128 = get_user_input("Enter a positive integer greater than 2: ");
    let primes: Vec<i128> = primes_up_to(user_input);
    println!("{:?}", primes);
}