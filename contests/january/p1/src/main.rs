/// Contest Problem 1
/// Minimum Number of Pushes to Type Word I
/// 
/// Given a string 'word' containing distinct lowercase English
/// letters, return the minimum number of pushes required to type
/// 'word' after remapping the keys on a default phone keyboard
/// following such:
/// 
/// You can remap the keys numbered 2 to 9 to distinct collections
/// of letters. The keys can be remapped to any amount of letters,
/// but each letter must be mapped to exactly one key. You need to
/// find the minimum number of times the keys will be pushed to
/// type the string 'word'.
/// 
/// Return the minimum number of pushes needed to type 'word'
/// after remapping the keys.
fn main() {
    let s1 = "abcde".to_string();
    let s2 = "xcydefghij".to_string();
    let s3 = "amrvxnhsewkoipjyuclgtdbfq".to_string();

    println!("abcde: {}, expected 5", Solution::minimum_pushes(s1));
    println!("xcydefghij: {}, expected 12", Solution::minimum_pushes(s2));
    println!("amrvxnhsewkoipjyuclgtdbfq: {}, expected 52", Solution::minimum_pushes(s3));
}


struct Solution;
impl Solution {

    /// all letters are unique, 1<=word.length<=26
    pub fn minimum_pushes(word: String) -> i32 {
        // the first 8 letters are mapped to 2,3,4,5,6,7,8,9 and
        // only require one push
        // the next 8 letters are mapped to 22,33,44,55,66,77,88,99
        // and require two pushes
        // the next 8 letters are mapped to 222,333,444,555,666,777,888,999
        // and require three pushes
        // the next 2 letters are mapped to 2222,3333 and require four pushes

        let mut pushes = 0;
        println!("word: {} length: {}", word, word.len());
        for i in 0..word.len() {
            if i < 8 {
                pushes += 1;
                continue;
            } else if i < 16 {
                pushes += 2;
                continue;
            } else if i < 24 {
                pushes += 3;
            } else {
                pushes += 4;
            }
        }

        pushes
        
    }
}