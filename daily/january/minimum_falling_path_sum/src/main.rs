/// Given an n x n array of integers 'matrix', return the minimum sum
/// of any falling path through 'matrix'.
/// 
/// A falling path starts at any element in the first row and chooses
/// the element in the next row that is either directly below or diagonally
/// left / right. Specifically, the next element from position (x, y) will be
/// (x, y + 1), (x + 1, y + 1), or (x - 1, y + 1). (Always down one row.)
fn main() {
    let t1 = vec![vec![2, 1, 3], vec![6, 5, 4], vec![7, 8, 9]];
    let t2 = vec![vec![-19, 57], vec![-40, -5]];

    println!("t1: {} (expected 13)", Solution::min_falling_path_sum(t1));
    println!("t2: {} (expected -59)", Solution::min_falling_path_sum(t2));
}

struct Solution;
impl Solution {
    pub fn min_falling_path_sum(matrix: Vec<Vec<i32>>) -> i32 {
        // by definition, a matrix's composite vectors are of equal length
        // so we can assume that the matrix is square
        let mut running_totals: Vec<Vec<i32>> = vec![vec![0; matrix.len()]; matrix.len()];

        // initialize the first row of the running totals
        for value in 0..matrix.len() {
            running_totals[0][value] = matrix[0][value];
        }

        // loop through matrix beginning at the second row
        for row in 1..matrix.len() {
            // loop through each column
            for (col, _) in matrix[0].iter().enumerate() {
                // start with the value above the current value
                let mut start = running_totals[row - 1][col];
                println!("start: {}", start);

                // if the current column is not the first column
                if col > 0 {
                    // compare the value to the left of the current value
                    start = std::cmp::min(start, running_totals[row - 1][col - 1]);
                }

                // if the current column is not the last column
                if col < matrix.len() - 1 {
                    // compare the value to the right of the current value
                    start = std::cmp::min(start, running_totals[row - 1][col + 1]);
                }

                // add the current value to the running total
                running_totals[row][col] = start + matrix[row][col];

                println!("row: {}, col: {}, value: {}", row, col, running_totals[row][col]);
            }
        }

        // return the minimum value in the last row of the running totals
        *running_totals.last().unwrap().iter().min().unwrap()
    }
}

