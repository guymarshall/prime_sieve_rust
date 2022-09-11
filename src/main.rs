mod math;
mod user_input;

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter a positive integer: ");
    let numbers: Vec<i32> = math::number_to_vector(user_input);
    let mut numbers_to_remove: Vec<i32> = Vec::new();

    let ceiling_root: i32 = (user_input as f64).sqrt().ceil() as i32;

    for i in 2..ceiling_root + 1 {
        for j in i..user_input + 1 {
            numbers_to_remove.push(i * j);
        }
    }

    for number in numbers_to_remove.iter() {
        println!("Number: {}", number);
    }

    numbers_to_remove.sort_unstable();
    numbers_to_remove.dedup();
    numbers_to_remove.retain(|x| *x <= user_input);

    println!("Numbers to remove: {:?}", numbers_to_remove);
}
