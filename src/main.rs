mod math;
mod user_input;

fn main() {
    let user_input: i32 = user_input::get_user_input("Enter a positive integer: ");
    let numbers: Vec<i32> = math::number_to_vector(user_input);

    println!("Numbers: {:?}", numbers);
}
