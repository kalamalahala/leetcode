fn main() {
    let account_totals = vec![vec![102, 25, 199], vec![4, 15, 201, 23], vec![1, 2, 3]];
    println!("Max wealth: {}", max_wealth(account_totals));
}

fn max_wealth(account_list: Vec<Vec<i32>>) -> i32 {
    let mut max = 0;
    let mut runsum = 0;

    for account in account_list {
        for balance in account {
            runsum += balance;
        }
        if runsum > max {
            max = runsum;
        }
        runsum = 0;
    }

    max
}
