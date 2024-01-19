/// Gas Station
/// https://leetcode.com/problems/gas-station/
/// 
/// There are 'n' gas stations along a circular route, where the amount
/// of gas to travel from station 'i' to the next station 'i + 1' is 'gas[i]'.
/// You have a car with an empty tank that can hold an unlimited amount of gas.
/// 
/// Start at any gas station and travel around the circuit once in the clockwise direction.
/// Return the starting gas station's index if you can travel around the circuit once, otherwise return -1.
fn main() {
    let gas = vec![1,2,3,4,5];
    let cost = vec![3,4,5,1,2];
    let result = Solution::can_complete_circuit(gas, cost);
    println!("result: {:?}", result);

    let gas = vec![2,3,4];
    let cost = vec![3,4,3];
    let result = Solution::can_complete_circuit(gas, cost);
    println!("result: {:?}", result);
}

struct Solution;
impl Solution {
    /// Since there is only one solution or none, and each station is visited,
    /// we only need to check if the total gas is greater than or equal to the total cost.
    /// If it is, then there is a solution, otherwise there is no solution.
    /// 
    /// To find the starting station, we increase the starting station by one
    /// when the current tank is less than 0. Because the current tank is less than 0,
    /// it means that the previous starting station cannot reach the current station.
    /// So we need to start from the next station.

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut curr_tank = 0;
        let mut total_tank = 0;
        let mut start_station = 0;

        for i in 0..gas.len() {
            curr_tank += gas[i] - cost[i];
            total_tank += gas[i] - cost[i];
            if curr_tank < 0 {
                start_station = i + 1;
                curr_tank = 0;
            }
        }

        if total_tank >= 0 {
            return start_station as i32;
        } else {
            return -1;
        }
    }
}