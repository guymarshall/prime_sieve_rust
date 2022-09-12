pub fn number_to_vector(number: i32) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();

    for i in 1..number + 1 {
        numbers.push(i);
    }

    numbers
}
