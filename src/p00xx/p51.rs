//!
//! N-Queens
//!
//! https://leetcode.com/problems/n-queens/
//!
//! The n-queens puzzle is the problem of placing n queens on an n×n chessboard such that no two queens attack each other.
//!
//! ![](https://assets.leetcode.com/uploads/2018/10/12/8-queens.png)
//!
//! Given an integer n, return all distinct solutions to the n-queens puzzle.
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
pub type Output = Vec<Vec<String>>;

pub trait Solution {
    fn solve_n_queens(&self, n: i32) -> Vec<Vec<String>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: DFS with 2D chessboard.
pub struct Solution0;
impl Solution for Solution0 {

    fn solve_n_queens(&self, n: i32) -> Vec<Vec<String>> {

        let n = n as usize;

        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut chessboard = vec![vec!['?'; n]; n];
        dfs_2d(&mut chessboard, &mut ans);
        ans
    }
}

fn dfs_2d(chessboard: &mut [Vec<char>], ans: &mut Vec<Vec<String>>) {

    for i in 0..chessboard.len() {
        
        let mut dot_counter = 0;
        for j in 0..chessboard.len() {

            match chessboard[i][j] {
                | '?' => {
                    // try fill 'Q'
                    if is_can_fill_q_2d(chessboard, i, j) {
                        chessboard[i][j] = 'Q';
                        dfs_2d(chessboard, ans);
                    }

                    // 'Q' has been tried.
                    // Now '.' is the only choose left.
                    chessboard[i][j] = '.';
                    dfs_2d(chessboard, ans);
                    chessboard[i][j] = '?';
                    return
                },
                | '.' => dot_counter += 1,
                | _   => {},
            }
        }

        if dot_counter == chessboard.len() { return }
    }


    let candidate: Vec<String> = chessboard.iter()
        .map(|s| s.iter().collect())
        .collect();
    ans.push(candidate);
}

fn is_can_fill_q_2d(chessboard: &[Vec<char>], x: usize, y: usize) -> bool {

    let len = chessboard.len();

    // check x-th row
    for j in 0..len {
        if chessboard[x][j] == 'Q' { return false }
    }

    // check y-th column
    for i in 0..len {
        if chessboard[i][y] == 'Q' { return false }
    }

    // check diagonal
    {
        let mut x = x + 1;
        let mut y = y as i32 - 1;

        while x < len && y >= 0 {
            if chessboard[x][y as usize] == 'Q' { return false }
            x += 1;
            y -= 1;
        }
    }

    {
        let mut x = x as i32 - 1;
        let mut y = y + 1;

        while x >= 0 && y < len {
            if chessboard[x as usize][y] == 'Q' { return false }
            x -= 1;
            y += 1;
        }
    }

    {
        let mut x = x as i32 - 1;
        let mut y = y as i32 - 1;

        while x >= 0 && y >= 0 {
            if chessboard[x as usize][y as usize] == 'Q' { return false }
            x -= 1;
            y -= 1;
        }
    }
    
    true
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: DFS with 1d chessboard.
pub struct Solution1;

const NO_QUEUE: usize = 10000;
impl Solution for Solution1 {

    fn solve_n_queens(&self, n: i32) -> Vec<Vec<String>> {

        let n = n as usize;
        let mut ans: Vec<Vec<String>> = Vec::new();

        // Use 1d array to represent the chessboard.
        // the i-th element is the column index of the i-th row for an Queue in chessboard.
        // NO_QUEUE means there is no Queue in that row.
        let mut chessboard = vec![NO_QUEUE; n];

        dfs_1d(&mut chessboard, 0, &mut ans);
        ans
    }
}

fn dfs_1d(chessboard: &mut [usize], row: usize, ans: &mut Vec<Vec<String>>) {

    if row == chessboard.len() {
        // an answer is found
        let candidate: Vec<String> = chessboard.iter()
            .map(|&queue_location| {
                let mut row = vec!['.'; chessboard.len()];
                row[queue_location] = 'Q';
                row.into_iter().collect()
            }).collect();
        ans.push(candidate);
    } else {
        for column in 0..chessboard.len() {
            if is_can_fill_q_1d(chessboard, row, column) {
                // try placing the Queue.
                chessboard[row] = column;
                // search for next row.
                dfs_1d(chessboard, row + 1, ans);
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

