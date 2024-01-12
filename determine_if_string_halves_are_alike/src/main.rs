fn main() {
    /*
    You are given a string 's' of even length. Split the string into two halves of equal lengths
    and let 'a' be the first half and 'b' be the second half. Two strings are *alike* if they have
    the same number of vowels ('a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'). Notice that 's'
    contains uppercase and lowercase letters.
     */

    let s1 = "book".to_string();
    let s2 = "textbook".to_string();
    let s3 = "MerryChristmas".to_string();
    let s4 = "AbCdEfGh".to_string();
    let s5 = "Uo".to_string();

    println!("s1: {}", Solution::halves_are_alike(s1));
    println!("s2: {}", Solution::halves_are_alike(s2));
    println!("s3: {}", Solution::halves_are_alike(s3));
    println!("s4: {}", Solution::halves_are_alike(s4));
    println!("s5: {}", Solution::halves_are_alike(s5));
}

struct Solution;
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        let mut s = s.to_lowercase();
        let second_half = s.split_off(s.len() / 2);
        let vowel_reference = vec!['a', 'e', 'i', 'o', 'u'];
        let mut first_half_vowel_count = 0;
        let mut second_half_vowel_count = 0;

        for letter in s.chars() {
            if vowel_reference.contains(&letter) {
                first_half_vowel_count += 1;
            }
        }

        for letter in second_half.chars() {
            if vowel_reference.contains(&letter) {
                second_half_vowel_count += 1;
            }
        }

        first_half_vowel_count == second_half_vowel_count
    }
}