mod math;
mod user_input;

// fn main() {
//     let user_input: i32 = user_input::get_user_input("Enter a positive integer: ");
//     let mut numbers: Vec<i32> = math::number_to_vector(user_input);
//     numbers.remove(numbers.iter().position(|x| *x == 1).unwrap());
//     let mut numbers_to_remove: Vec<i32> = Vec::new();

//     let ceiling_root: i32 = (user_input as f64).sqrt().ceil() as i32;

//     for i in 2..ceiling_root + 1 {
//         for j in i..user_input + 1 {
//             numbers_to_remove.push(i * j);
//         }
//     }

//     numbers_to_remove.sort_unstable();
//     numbers_to_remove.dedup();
//     numbers_to_remove.retain(|x| *x <= user_input);

//     for number in numbers_to_remove {
//         if numbers.iter().any(|&i| i == number) {
//             numbers.remove(numbers.iter().position(|x| *x == number).unwrap());
//         }
//     }

//     println!("Prime numbers up to {}: {:?}", user_input, numbers);
// }

fn primes_up_to(number: i32) {
    let primes: Vec<i32> = (2..=number).filter(is_prime).collect::<Vec<i32>>();

    println!("{:?}", primes);
}

fn is_prime(number: &i32) -> bool {
    !(2..*number).any(|n| number % n == 0)
}

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter a positive integer greater than 2: ");
    primes_up_to(user_input);
}