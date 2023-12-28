fn main() {
    println!("Steps to reduce 14 to 0: {}", count_reduce(14));
}

fn count_reduce(n: i32) -> i32 {
    let mut step_count = 0;
    let mut current_number = n;

    while current_number > 0 {
        if current_number % 2 == 0 {
            current_number /= 2;
            step_count += 1;
        } else {
            current_number -= 1;
            step_count += 1;
        }
    }

    if current_number == 0 {
        step_count
    } else {
        0
    }
}