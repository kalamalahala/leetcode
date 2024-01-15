use std::vec;

/// Given an integer array 'matches' where 'matches[i] = [winner-i, loser-i]'
/// indicates that the 'i'th match was won by 'winner-i' and lost by 'loser-i',
/// return a list of size 2 where list[0] is a list of all players that
/// have not lost any matches, and list[1] is a list of all players that
/// have lost exactly one match. Both lists should be sorted in ascending order.
/// 
/// - Only consider players that have played at least one match.
/// - No two matches have the same result.
fn main() {
    let m1 = vec![vec![1,3],vec![2,3],vec![3,6],vec![5,6],vec![5,7],vec![4,5],vec![4,8],vec![4,9],vec![10,4],vec![10,9]];
    let m2 = vec![vec![2,3],vec![1,3],vec![5,4],vec![6,4]];

    println!("Expected [1,2,10], [4,5,7,8]: {:?}", Solution::find_winners(m1));
    println!("Expected [1,2,5,6], []: {:?}", Solution::find_winners(m2));

    
}

struct Solution;
impl Solution {
    pub fn find_winners(matches: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;


        let mut players: HashMap<i32, i32> = HashMap::new();
        let mut winners: HashMap<i32, i32> = HashMap::new();
        let mut losers: HashMap<i32, i32> = HashMap::new();

        for m in matches {
            let winner = m[0];
            let loser = m[1];

            let winner_count = winners.entry(winner).or_insert(0);
            *winner_count += 1;

            let loser_count = losers.entry(loser).or_insert(0);
            *loser_count += 1;

            players.entry(winner).or_insert(0);
            players.entry(loser).or_insert(0);
        }

        let mut zero_loss: Vec<i32> = Vec::new();
        let mut one_loss: Vec<i32> = Vec::new();

        for (p, _) in players {
            let winner_count = winners.entry(p).or_insert(0);
            let loser_count = losers.entry(p).or_insert(0);

            if *winner_count > 0 && *loser_count == 0 {
                zero_loss.push(p);
            } else if *winner_count >= 0 && *loser_count == 1 {
                one_loss.push(p);
            }
        }


        zero_loss.sort_unstable();
        one_loss.sort_unstable();

        vec![zero_loss, one_loss]
    }
}