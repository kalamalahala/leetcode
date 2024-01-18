/// Leetcode January Daily #18
/// Climbing Stairs
/// https://leetcode.com/problems/climbing-stairs/
fn main() {
    let n = 2;
    let result = Solution::climb_stairs(n);

    println!("{} stairs: {}", n, result);
}

struct Solution;
impl Solution {
    pub fn climb_stairs(n: i32) -> i32 {
        if n < 3 {
            return n;
        }

        let mut stairs = vec![0; n as usize];
        stairs[0] = 1;
        stairs[1] = 2;

        for i in 2..n as usize {
            stairs[i] = stairs[i - 1] + stairs[i - 2];
        }

        stairs[n as usize - 1]
    }
}