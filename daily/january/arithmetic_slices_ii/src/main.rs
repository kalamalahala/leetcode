fn main() {
    let t1 = vec![2, 4, 6, 8, 10]; // expect 7
    let t2 = vec![7, 7, 7, 7, 7]; // expect 16

    println!("{}", Solution::number_of_arithmetic_slices(t1));
    println!("{}", Solution::number_of_arithmetic_slices(t2));
}

struct Solution {}
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        let mut dp = vec![std::collections::HashMap::new(); nums.len()];
        let mut res = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                println!("current nums[i]: {}, nums[j]: {}", nums[i], nums[j]);
                let diff = nums[i] as i64 - nums[j] as i64;
                let count = *dp[j].get(&diff).unwrap_or(&0);
                res += count;
                *dp[i].entry(diff).or_insert(0) += count + 1;
            }
        }
        res
    }
}