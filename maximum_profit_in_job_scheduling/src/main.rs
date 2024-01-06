/**
 * We have n jobs, where every job is scheduled to be done from
 * startTime[i] to endTime[i], obtaining a profit of profit[i].
 * 
 * You're given the startTime , endTime and profit arrays, return
 * the maximum profit you can take such that there are no two jobs 
 * in the subset with overlapping time range.
 * 
 * If you choose a job that ends at time X you will be able to start
 * another job that starts at time X.
 * 
 * Example 1:
 * 
 * Input: startTime = [1,2,3,3], endTime = [3,4,5,6], profit = [50,10,40,70]
 * Output: 120
 * Explanation: The subset chosen is the first and fourth job.
 * Time range [1-3]+[3-6] , we get profit of 120 = 50 + 70.
 */
fn main() {
    let job_start_test_30 = vec![1,2,2,3];
    let job_end_test_30 = vec![2,5,3,4];
    let job_profit_test_30 = vec![3,4,1,2];

    println!("30: Maximum profit: {:#?}", max_profit(job_start_test_30, job_end_test_30, job_profit_test_30));
}


fn max_profit(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
    let mut jobs: Vec<_> = end_time.into_iter()
        .zip(start_time).zip(profit)
        .map(|((i, j), k)| (i, j, k))
        .collect();

    jobs.sort_unstable();

    let mut dp_table = Vec::with_capacity(jobs.len() + 1);
    dp_table.push(0);

    for (i, &(_, start, profit)) in jobs.iter().enumerate() {
        let job = jobs[..i].partition_point(|&(e, _, _)| e <= start);
        dp_table.push(dp_table[i].max(dp_table[job] + profit));
    }

    *dp_table.last().unwrap()
}
