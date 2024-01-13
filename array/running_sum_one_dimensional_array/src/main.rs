fn main() {
    let nums = vec![4, 15, 201, 23];
    running_sum(nums);
}

fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = [].to_vec();
    let mut runsum = 0;

    for item in nums {
        runsum += item;
        result.push(runsum);
    }

    println!("result: {:?}", result);
    result
}