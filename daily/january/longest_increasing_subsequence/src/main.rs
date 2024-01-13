fn main() {
    let nums1 = vec![10,9,2,5,3,7,101,18];
    let nums2 = vec![0,1,0,3,2,3];
    let nums3 = vec![7,7,7,7,7,7,7];


    println!("nums1: {}", length_of_lis(nums1));
    println!("nums2: {}", length_of_lis(nums2));
    println!("nums3: {}", length_of_lis(nums3));
}

/**
 * Given an integer array nums, return the length of the longest strictly increasing subsequence.
 */
fn length_of_lis(nums: Vec<i32>) -> i32 {
    use std::cmp::Ordering;

    assert!(!nums.is_empty());

    let mut seq = Vec::with_capacity(nums.len());
    seq.push(nums[0]);

    for &n in nums.iter().skip(1) {
        match n.cmp(seq.last().unwrap()) {
            Ordering::Equal => continue,
            Ordering::Greater => seq.push(n),
            Ordering::Less => match seq.binary_search(&n) {
                Ok(_) => {},
                Err(idx) => {
                    seq[idx] = n
                }
            }
        }
    }

    seq.len() as i32
}