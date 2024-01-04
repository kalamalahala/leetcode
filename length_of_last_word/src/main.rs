fn main() {
    let s1 = "Hello world!".to_string();
    let s2 = "Hello world".to_string();
    let s3 = "and I will walk 500 milessss".to_string();
    let s4 = "a".to_string();

    println!("s1: {}", length_of_last_word(s1));
    println!("s2: {}", length_of_last_word(s2));
    println!("s3: {}", length_of_last_word(s3));
    println!("s4: {}", length_of_last_word(s4));
}

fn length_of_last_word(s: String) -> i32 {
    s.split_whitespace().collect::<Vec<&str>>().last().unwrap().len() as i32
}
