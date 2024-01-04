use std::vec;

fn main() {
    println!("{:?}", fizz_buzz(30))
}


// given an integer n, return a string array answer (1-indexed) where:
// answer[i] == "FizzBuzz" if i is divisible by 3 and 5
// answer[i] == "Fizz" if i is divisible by 3
// answer[i] == "Buzz" if i is divisible by 5
// answer[i] == i if non of the above conditions are true

fn fizz_buzz(n: i32) -> Vec<String> {
    let mut result = vec![];

    for i in 1..=n {
        if i % 3 == 0 && i % 5 == 0 {
            result.push(String::from("FizzBuzz"));
        } else if i % 3 == 0 {
            result.push(String::from("Fizz"));
        } else if i % 5 == 0 {
            result.push(String::from("Buzz"));
        } else {
            result.push(i.to_string());
        }
    }

    result
}