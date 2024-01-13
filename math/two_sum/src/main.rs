fn main() {
    let n1 = vec![2, 7, 11, 15];
    let n2 = vec![3, 2, 4];
    let n3 = vec![3, 3];

    let t1 = 9;
    let t2 = 6;
    let t3 = 6;

    println!("n1: {:?}", two_sum(n1, t1));
    println!("n2: {:?}", two_sum(n2, t2));
    println!("n3: {:?}", two_sum(n3, t3));
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut result = Vec::new();

    let map = nums.iter().enumerate().map(|(i, &n)| (n, i)).collect::<std::collections::HashMap<i32, usize>>();

    for (i, &n) in nums.iter().enumerate() {
        let complement = target - n;
        if map.contains_key(&complement) && map[&complement] != i {
            result.push(i as i32);
            result.push(map[&complement] as i32);
            break;
        }
    }

    result
}