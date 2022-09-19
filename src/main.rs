use std::io;
// use rayon::prelude::*;

pub fn get_user_input(prompt: &str) -> usize {
    println!("{}", prompt);

    let mut user_input: String = String::new();

    io::stdin()
        .read_line(&mut user_input)
        .expect("Failed to read line");

    let number: usize = user_input.trim().parse().expect("Please enter an integer!");

    return number;
}

// fn primes_up_to(number: i32) {
//     let primes: Vec<i32> = (2..=number).into_par_iter().filter(is_prime).collect::<Vec<i32>>();

//     println!("{:?}", primes);
// }

// fn is_prime(number: &i32) -> bool {
//     !(2..*number).any(|n| number % n == 0)
// }

// fn main() {
//     let user_input: i32 = get_user_input("Enter a positive integer greater than 2: ");
//     primes_up_to(user_input);
// }

fn main() {
    let user_input: usize = get_user_input("Enter a positive integer greater than 2: ");

    let mut is_prime: Vec<bool> = vec![true; user_input];
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..user_input {
        if is_prime[i] {
            for j in 2..user_input / i {
                is_prime[i * j] = false;
            }
        }
    }
}