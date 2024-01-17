/// Leetcode Daily: January 17, 2024
/// Given an array of integers 'arr', return 'true' if
/// the number of occurences of each value in the array
/// is unique, or 'false' otherwise.
fn main() {
    let e1 = vec![1, 2, 2, 1, 1, 3];
    let e2 = vec![1, 2];
    let e3 = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];

    println!("e1: {} (expected: true)", Solution::unique_occurrences(e1));
    println!("e2: {} (expected: false)", Solution::unique_occurrences(e2));
    println!("e3: {} (expected: true)", Solution::unique_occurrences(e3));
}

struct Solution;
impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        use std::collections::HashMap;

        let mut counts: HashMap<i32, i32> = HashMap::new();
        for n in arr {
            let count = counts.entry(n).or_insert(0);
            *count += 1;
        }

        let mut occurences: Vec<i32> = counts.values().cloned().collect();
        occurences.sort();

        for i in 0..occurences.len() - 1 {
            if occurences[i] == occurences[i + 1] {
                return false;
            }
        }

        true
    }
}