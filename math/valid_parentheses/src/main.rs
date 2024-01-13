fn main() {
    let s = String::from("()[]{}");
    let s2 = String::from("([)]");
    let s3 = String::from("{(oh hey)}");
    println!("s1 is valid: {}", valid_parens(s));
    println!("s2 is valid: {}", valid_parens(s2));
    println!("s3 is valid: {}", valid_parens(s3));
}


fn valid_parens(s: String) -> bool {
    let mut temp_stack: Vec<char> = Vec::new();

    for character in s.chars() {
        match character {
            '(' => temp_stack.push(')'),
            '[' => temp_stack.push(']'),
            '{' => temp_stack.push('}'),
            ')' => {
                if temp_stack.pop() != Some(')') {
                    return false;
                }
            },
            ']' => {
                if temp_stack.pop() != Some(']') {
                    return false;
                }
            },
            '}' => {
                if temp_stack.pop() != Some('}') {
                    return false;
                }
            },
            _ => {
                continue;
            }
        }
    }

    temp_stack.is_empty()
    
}