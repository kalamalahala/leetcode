/// Product of Array Except Self
/// https://leetcode.com/problems/product-of-array-except-self/
/// 
/// Given an integer array 'nums', return array 'answer' such that
/// 'answer[i]' is equal to the product of all the elements of 'nums'
/// except 'nums[i]'.
fn main() {
    let a1 = vec![1, 2, 3, 4];
    let a2 = vec![-1, 1, 0, -3, 3];
    let a3 = vec![0, 0];
    let a4 = vec![1, 0];
    let a5 = vec![0, 1];
    let a6 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("Op 1: {:?} (expect: [24, 12, 8, 6])", Solution::product_except_self (a1));
    println!("Op 2: {:?} (expect: [0, 0, 9, 0, 0])", Solution::product_except_self(a2));
    println!("Op 3: {:?} (expect: [0, 0])", Solution::product_except_self (a3));
    println!("Op 4: {:?} (expect: [0, 1])", Solution::product_except_self (a4));
    println!("Op 5: {:?} (expect: [1, 0])", Solution::product_except_self (a5));
    println!("Op 6: {:?} (expect: [3628800, 1814400, 1209600, 907200, 725760, 604800, 518400, 453600, 403200, 362880])", Solution::product_except_self(a6));
}

struct Solution;
impl Solution {
    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let len = nums.len(); // length of nums
        let mut result = vec![1; nums.len()]; // result vector
        let mut left = 1; // left product
        let mut right = 1; // right product

        for i in 0..len { // loop through nums
            result[i] *= left; // multiply left product to result
            left *= nums[i]; // update left product
            result[len - 1 - i] *= right; // multiply right product to result
            right *= nums[len - 1 - i]; // update right product
        }


        result
    }
}
