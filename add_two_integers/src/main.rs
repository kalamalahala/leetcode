fn main() {
    println!("The sum of 5 and 7 is {}", add_two_integers(5, 7));
    println!("The sum of -83 and 32 is {}", add_two_integers(-5, 32));

    // error out of bounds
    println!("The sum of 5 and 1320 is {}", add_two_integers(5, 1320));
    ()
}

fn add_two_integers(num1: i32, num2: i32) -> i32 {
    if num1 < -100 || num2 > 100 {
        panic!("Integer out of bounds for problem!")
    }
    num1 + num2
}
