/// Two strings are considered close if you can attain one from the other using
/// the following operations:
/// - Swap any two existing characters.
/// - Transform every occurrence of one existing character into another existing
///   character, and do the same with the other character.
///   (All existing A's into B's, and all existing B's into A's).
/// You can use the operations on either string as many times as necessary.
/// Given two strings, word1 and word2, return true if word1 and word2 are close,
/// and false otherwise.
fn main() {
    let test1 = Solution::close_strings("abc".to_string(), "bca".to_string());
    let test2 = Solution::close_strings("a".to_string(), "aa".to_string());
    let test3 = Solution::close_strings("cabbba".to_string(), "abbccc".to_string());

    println!("test1: {} (expected true)", test1);
    println!("test2: {} (expected false)", test2);
    println!("test3: {} (expected true)", test3);
}

struct Solution;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        if word1.len() != word2.len() {
            return false;
        }
        use std::collections::HashMap;
        let char_map1: HashMap<char, i32> = Solution::get_char_map(word1);
        let char_map2: HashMap<char, i32> = Solution::get_char_map(word2);

        /*
        [src\main.rs:30] &char_map1 = {
            'b': 3,
            'a': 2,
            'c': 1,
        }
        [src\main.rs:31] &char_map2 = {
            'c': 3,
            'a': 1,
            'b': 2,
        }
        */
        let mut char_map1_keys: Vec<char> = char_map1.keys().cloned().collect();
        let mut char_map2_keys: Vec<char> = char_map2.keys().cloned().collect();
        char_map1_keys.sort();
        char_map2_keys.sort();

        if char_map1_keys != char_map2_keys {
            return false;
        }

        let mut char_map1_values: Vec<i32> = char_map1.values().cloned().collect();
        let mut char_map2_values: Vec<i32> = char_map2.values().cloned().collect();
        char_map1_values.sort();
        char_map2_values.sort();

        if char_map1_values != char_map2_values {
            return false;
        }

        true
    }

    fn get_char_map(word: String) -> std::collections::HashMap<char, i32> {
        let mut char_map = std::collections::HashMap::new();
        for c in word.chars() {
            let count = char_map.entry(c).or_insert(0);
            *count += 1;
        }
        char_map
    }
}
