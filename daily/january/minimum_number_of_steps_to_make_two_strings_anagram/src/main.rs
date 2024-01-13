/// You are given two strings of the same length, 's' and 't'.
/// In one step, you can choose any character of 't' and replace
/// it with another character.
/// 
/// Return the minimum number of steps to make 't' an anagram of 's'.
/// 
/// An 'Anagram' of a string is a string that contains the same characters
/// with a different (or the same) ordering.
/// 
/// Topics: Hash Table, String, Counting
fn main() {
    let s1 = String::from("bab");
    let s2 = String::from("aba");
    let s3 = String::from("leetcode");
    let s4 = String::from("practice");
    let s5 = String::from("anagram");
    let s6 = String::from("mangaar");

    println!("bab - aba: {} (expected 1)", Solution::min_steps(s1, s2));
    println!("leetcode - practice: {} (expected 5)", Solution::min_steps(s3, s4));
    println!("anagram - mangaar: {} (expected 0)", Solution::min_steps(s5, s6));
}

struct Solution;
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        use std::collections::HashMap;
        let mut s_map: HashMap<char, i32> = HashMap::with_capacity(s.len());
        let mut t_map: HashMap<char, i32> = HashMap::with_capacity(t.len());

        for c in s.chars() {
            let count = s_map.entry(c).or_insert(0);
            *count += 1;
        }

        for c in t.chars() {
            let count = t_map.entry(c).or_insert(0);
            *count += 1;
        }

        let mut steps = 0;
        for (k, v) in s_map.iter() {
            let t_count = t_map.entry(*k).or_insert(0);
            if v > t_count {
                steps += *v - *t_count;
            }
        }

        steps
    }
}
