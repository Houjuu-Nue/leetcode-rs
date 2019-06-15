//!
//! Sudoku Solver
//!
//! https://leetcode.com/problems/sudoku-solver/
//!
//! Write a program to solve a Sudoku puzzle by filling the empty cells.
//!
//! A sudoku solution must satisfy **all of the following rules**:
//!
//! 1. Each of the digits `1-9` must occur exactly once in each row.
//!
//! 2. Each of the digits `1-9` must occur exactly once in each column.
//!
//! 3. Each of the the digits `1-9` must occur exactly once in each of the 9 `3x3` sub-boxes of the grid.
//!
//! Empty cells are indicated by the character `'.'`.
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png)
//!
//! A sudoku puzzle
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/3/31/Sudoku-by-L2G-20050714_solution.svg/250px-Sudoku-by-L2G-20050714_solution.svg.png)
//!
//! ...and its solution numbers marked in red.
//!
//! **Note:**
//! - The given board contain only digits `1-9` and the character `'.'`.
//!
//! - You may assume that the given Sudoku puzzle will have a single unique solution.
//!
//! - The given board size is always `9x9`.
//!


pub type Input = Vec<Vec<char>>;
pub trait Solution {
    fn solve_sudoku(&self, board: &mut Vec<Vec<char>>);
}

// -----------------------------------------------------------------------------
/// Approach 0: DFS.
pub struct Solution0;
impl Solution for Solution0 {

    fn solve_sudoku(&self, board: &mut Vec<Vec<char>>) {

        dfs(board);
    }
}

fn dfs(board: &mut [Vec<char>]) -> bool {

    const CANDIDATE: &'static str = "123456789";

    for i in 0..9 {
        for j in 0..9 {

            if board[i][j] == '.' {

                for ch in CANDIDATE.chars() {

                    // try fill the character
                    board[i][j] = ch;
                    
                    // recursive test
                    if is_fill_ok(board, i, j, ch) && dfs(board) {
                        return true
                    }

                    // test failed, restore it to unfill.
                    board[i][j] = '.';
                }

                return false
            }
        }
    }

    true
}

fn is_fill_ok(board: &[Vec<char>], x: usize, y: usize, value: char) -> bool {

    // check x-th row
    for j in 0..9 {
        if j != y && board[x][j] == value { return false }
    }

    // check y-th column
    for i in 0..9 {
        if i != x && board[i][y] == value { return false }
    }

    // check sub-box
    let row    = (x / 3) * 3;
    let column = (y / 3) * 3;
    for i in row..(row + 3) {
        for j in column..(column + 3) {
            if i != x && j != y && board[i][j] == value { return false }
        }
    }

    true
}
// -----------------------------------------------------------------------------

