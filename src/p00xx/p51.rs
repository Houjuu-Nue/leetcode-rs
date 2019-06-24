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
/// Approach 0: Brute Force and recursion.
pub struct Solution0;
impl Solution for Solution0 {

    fn solve_n_queens(&self, n: i32) -> Vec<Vec<String>> {

        let n = n as usize;

        let mut ans: Vec<Vec<String>> = Vec::new();
        let mut checkboard = vec![vec!['?'; n]; n];
        dfs(&mut checkboard, &mut ans);
        ans
    }
}

fn dfs(checkboard: &mut [Vec<char>], ans: &mut Vec<Vec<String>>) {

    for i in 0..checkboard.len() {
        
        let mut dot_counter = 0;
        for j in 0..checkboard.len() {

            match checkboard[i][j] {
                | '?' => {
                    // try fill 'Q'
                    if is_can_fill_q(checkboard, i, j) {
                        checkboard[i][j] = 'Q';
                        dfs(checkboard, ans);
                    }

                    checkboard[i][j] = '.';
                    dfs(checkboard, ans);
                    checkboard[i][j] = '?';
                    return
                },
                | '.' => dot_counter += 1,
                | _   => {},
            }
        }

        if dot_counter == checkboard.len() { return }
    }


    let candidate: Vec<String> = checkboard.iter()
        .map(|s| s.iter().collect())
        .collect();
    ans.push(candidate);
}

fn is_can_fill_q(checkboard: &[Vec<char>], x: usize, y: usize) -> bool {

    let len = checkboard.len();

    // check x-th row
    for j in 0..len {
        if checkboard[x][j] == 'Q' { return false }
    }

    // check y-th column
    for i in 0..len {
        if checkboard[i][y] == 'Q' { return false }
    }

    // check diagonal
    {
        let mut x = x + 1;
        let mut y = y as i32 - 1;

        while x < len && y >= 0 {
            if checkboard[x][y as usize] == 'Q' { return false }
            x += 1;
            y -= 1;
        }
    }

    {
        let mut x = x as i32 - 1;
        let mut y = y + 1;

        while x >= 0 && y < len {
            if checkboard[x as usize][y] == 'Q' { return false }
            x -= 1;
            y += 1;
        }
    }

    {
        let mut x = x as i32 - 1;
        let mut y = y as i32 - 1;

        while x >= 0 && y >= 0 {
            if checkboard[x as usize][y as usize] == 'Q' { return false }
            x -= 1;
            y -= 1;
        }
    }
    
    true
}
// -----------------------------------------------------------------------------

