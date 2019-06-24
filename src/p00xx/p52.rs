//!
//! N-Queens II
//!
//! https://leetcode.com/problems/n-queens-ii/
//!
//! The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
//!
//! ![](https://assets.leetcode.com/uploads/2018/10/12/8-queens.png)
//!
//! Given an integer n, return the number of distinct solutions to the n-queens puzzle.
//!
//! Each solution contains a distinct board configuration of the n-queens' placement, where `'Q'` and `'.'` both indicate a queen and an empty space respectively.
//!
//! **Example:**
//! ```text
//! Input: 4
//! Output: [
//!  [".Q..",  // Solution 1
//!   "...Q",
//!   "Q...",
//!   "..Q."],
//!
//! ["..Q.",  // Solution 2.
//!  "Q...",
//!  "...Q",
//!  ".Q.."].
//! ].
//!
//! Explanation: There exist two distinct solutions to the 4-queens puzzle as shown above.
//! ```
//!



pub type Input  = i32;
pub type Output = i32;

pub trait Solution {
    fn total_n_queens(&self, n: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Simpilified vertion of DFS with 1d chessboard.
pub struct Solution0;

const NO_QUEUE: usize = 10000;
impl Solution for Solution0 {

    fn total_n_queens(&self, n: i32) -> i32 {

        let n = n as usize;
        let mut count = 0;

        // Use 1d array to represent the chessboard.
        // the i-th element is the column index of the i-th row for an Queue in chessboard.
        // NO_QUEUE means there is no Queue in that row.
        let mut chessboard = vec![NO_QUEUE; n];

        dfs_1d(&mut chessboard, 0, &mut count);
        count as i32
    }
}

fn dfs_1d(chessboard: &mut [usize], row: usize, count: &mut usize) {

    if row == chessboard.len() {
        // an answer is found
        *count += 1;
    } else {
        for column in 0..chessboard.len() {
            if is_can_fill_q_1d(chessboard, row, column) {
                // try placing the Queue.
                chessboard[row] = column;
                // search for next row.
                dfs_1d(chessboard, row + 1, count);
                // restore placement.
                chessboard[row] = NO_QUEUE;
            }
        }
    }
}

fn is_can_fill_q_1d(chessboard: &[usize], row: usize, column: usize) -> bool {

    // there is no conflit in rows

    for i in 0..chessboard.len() {
        // check y-th column.
        if chessboard[i] == column {
            return false
        }
        // check diagonal
        if (i as i32 - row as i32).abs() == (chessboard[i] as i32 - column as i32).abs() {
            return false
        }
    }

    true
}
// -----------------------------------------------------------------------------

