/// House Robber
/// https://leetcode.com/problems/house-robber/
/// 
/// You are a professional robber planning to rob houses along
/// a street. Each house has a certain amount of money stashed,
/// the only constraint stopping you from robbing each of them 
/// is that adjacent houses have security systems connected and
/// it will automatically contact the police if two adjacent
/// houses were broken into on the same night.
/// 
/// Given an integer array 'nums' representing the amount
/// of money of each house, return the maximum amount of money you
/// can rob tonight without alerting the police.
fn main() {
    let h1 = vec![1, 2, 3, 1];
    let h2 = vec![2, 7, 9, 3, 1];
    let h3 = vec![2, 1, 1, 2];

    println!("h1: {} expected: 4", Solution::rob(h1));
    println!("h2: {} expected: 12", Solution::rob(h2));
    println!("h3: {} expected: 4", Solution::rob(h3));
}

struct Solution;
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        if nums.len() == 1 {
            return nums[0];
        }

        let mut dp = vec![0; nums.len()];
        dp[0] = nums[0];
        dp[1] = std::cmp::max(nums[0], nums[1]);

        for index in 2..nums.len() {
            dp[index] = std::cmp::max(dp[index - 1], dp[index - 2] + nums[index]);
        }

        dp.last().unwrap().to_owned()
    }
}