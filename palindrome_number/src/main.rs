fn main() {
    // given an integer x, return true if x is palindrome integer and false otherwise
    println!("121: {}", is_palindrome(121));
    println!("-121: {}", is_palindrome(-121));
    println!("10: {}", is_palindrome(10));
    println!("0: {}", is_palindrome(0));
    println!("123454321: {}", is_palindrome(123454321));
    println!("8675309: {}", is_palindrome(8675309));
}

fn is_palindrome(x: i32) -> bool {
    if x < 0 || (x % 10 == 0 && x != 0) { 
        return false;
    }

    if x < 10 {
        return true;
    }

    let mut num = x;
    let mut reversed = 0;

    while num > reversed {
        reversed = reversed * 10 + num % 10;
        num /= 10;
    }

    num == reversed || num == reversed / 10
}
