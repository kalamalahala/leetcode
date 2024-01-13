fn main() {
    let test_vec = vec![13,7,13,7,13,7,13,13,7,1,1,1,2,2,2,2,2,2,3,3,3,3,3,3,3,4,4,4,4,4,6,6,6,6,6,6,6,6];
    println!("The minimum number of operations to make the array empty is {}", min_operations(test_vec));
}

/**
 * Given a 0-indexed int array nums of positive integers,
 * apply either of the two following operations any number of times until the array is empty:
 * 1. Choose two elements with equal values and delete them from the array.
 * 2. Choose three elements with equal values and delete them from the array.
 * 
 * Return the minimum number of operations to make the array empty.
 * Return -1 if it is impossible to empty the array.
 */
fn min_operations(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let mut status = 0;

    // create a hashmap to store the number of occurences of each number
    let mut num_map = std::collections::HashMap::new();
    for num in nums {
        let count = num_map.entry(num).or_insert(0);
        *count += 1;
    }

    for (num, count) in num_map {
        if count == 1 {
            status = -1;
            break;
        }
        let mut ops_required = 0;

        if count == 2 {
            status += 1;
            ops_required = 1;
            println!("{}: {} ({} ops required)", num, count, ops_required);
            continue;
        }

        if count % 3 == 0 { 
            ops_required = count / 3;
            status += count / 3;
        }

        if count % 3 == 2 {
            status += 1 + (count - 1) / 3;
            ops_required = 1 + (count - 1) / 3;
        }

        if count % 3 == 1 { // delete as many as possible in groups of 3 until only an even number remains, then delete in groups of 2
            let mut remaining = count;
            while remaining % 3 != 0 {
                remaining -= 2;
                ops_required += 1;
                status += 1;
            }
            ops_required += remaining / 3;
            status += remaining / 3;
            }

        println!("{}: {} ({} ops required)", num, count, ops_required);
    }

    status
}
