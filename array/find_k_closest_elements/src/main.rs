/// Given a sorted array, two integers k and x, find the k closest elements to x in the array.
/// The result should also be sorted in ascending order. If there is a tie, the smaller elements are always preferred.
/// | a - x | < | b - x |, then a is closer to x than b.
/// | a - x | == | b - x |, then a is closer to x than b if a < b.
fn main() {
    let a1 = vec![1, 2, 3, 4, 5]; // k = 4, x = 3, output: [1, 2, 3, 4]
    let a2 = vec![1, 2, 3, 4, 5]; // k = 4, x = -1, output: [1, 2, 3, 4]

    println!("expected: [1, 2, 3, 4], actual: {:?}", Solution::find_closest_elements(a1, 4, 3));
    println!("expected: [1, 2, 3, 4], actual: {:?}", Solution::find_closest_elements(a2, 4, -1));
}

struct Solution;
impl Solution {
    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        // init binary search
        let num_elements = arr.len(); // array size
        let k = k as usize; // how many elements to return
        
        // start at index zero, end at k less than the array size
        let (mut open, mut close) = (0usize, num_elements - k);
        let mut midpoint: usize;

        // binary search
        while open < close {
            // set midpoint
            midpoint = (open + close) / 2;

            // x is the root of the search
            // check if x is closer to the left or right
            if x - arr[midpoint] > arr[midpoint + k] - x {
                // x is closer to the right
                open = midpoint + 1;
            } else {
                // x is closer to the left
                close = midpoint;
            }
        }
        
        // return the k elements
        arr[open..open + k].to_vec()
    }

}