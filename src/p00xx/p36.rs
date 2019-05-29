//!
//! Valid Sudoku
//!
//! https://leetcode.com/problems/valid-sudoku/
//!
//! Determine if a 9x9 Sudoku board is valid.
//!
//! Only the filled cells need to be **validated according to the following rules**:
//!
//! 1. Each row must contain the digits `1-9` without repetition.
//!
//! 2. Each column must contain the digits `1-9` without repetition.
//!
//! 3. Each of the 9 `3x3` sub-boxes of the grid must contain the digits `1-9` without repetition.
//!
//! ![](https://upload.wikimedia.org/wikipedia/commons/thumb/f/ff/Sudoku-by-L2G-20050714.svg/250px-Sudoku-by-L2G-20050714.svg.png).
//!
//! A partially filled sudoku which is valid.
//!
//! The Sudoku board could be partially filled, where empty cells are filled with the character `'.'`.
//!
//! **Example 1:**
//! ```text
//! Input:
//! [
//!   ["5","3",".",".","7",".",".",".","."],
//!   ["6",".",".","1","9","5",".",".","."],
//!   [".","9","8",".",".",".",".","6","."],
//!   ["8",".",".",".","6",".",".",".","3"],
//!   ["4",".",".","8",".","3",".",".","1"],
//!   ["7",".",".",".","2",".",".",".","6"],
//!   [".","6",".",".",".",".","2","8","."],
//!   [".",".",".","4","1","9",".",".","5"],
//!   [".",".",".",".","8",".",".","7","9"]
//! ]
//! Output: true
//! ```
//!
//! **Example 2:**
//! ```text
//! Input:
//! [
//!   ["8","3",".",".","7",".",".",".","."],
//!   ["6",".",".","1","9","5",".",".","."],
//!   [".","9","8",".",".",".",".","6","."],
//!   ["8",".",".",".","6",".",".",".","3"],
//!   ["4",".",".","8",".","3",".",".","1"],
//!   ["7",".",".",".","2",".",".",".","6"],
//!   [".","6",".",".",".",".","2","8","."],
//!   [".",".",".","4","1","9",".",".","5"],
//!   [".",".",".",".","8",".",".","7","9"]
//! ]
//! Output: false
//! Explanation: Same as Example 1, except with the 5 in the top left corner being 
//!     modified to 8. Since there are two 8's in the top left 3x3 sub-box, it is invalid.
//! ```
//!
//! **Note:**
//! 
//! - A Sudoku board (partially filled) could be valid but is not necessarily solvable.
//!
//! - Only the filled cells need to be validated according to the mentioned rules.
//!
//! - The given board contain only digits `1-9` and the character `'.'`.
//!
//! - The given board size is always `9x9`.
//!



pub type Input  = Vec<Vec<char>>;
pub type Output = bool;

pub trait Solution {
    fn is_valid_sudoku(&self, board: Vec<Vec<char>>) -> bool;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn is_valid_sudoku(&self, board: Vec<Vec<char>>) -> bool {

        use std::collections::HashSet;

        // verify rows
        for rows in board.iter() {
            let mut set = HashSet::new();
            for ch in rows.iter() {
                if *ch == '.' { continue }

                if set.contains(ch) {
                    return false
                } else {
                    set.insert(ch);
                }
            }
        }
        
        // verify columns
        for column in 0..9 {
            let mut set = HashSet::new();
            for row in 0..9 {
                let ch = board[row][column];
                if ch == '.' { continue }

                if set.contains(&ch) {
                    return false
                } else {
                    set.insert(ch);
                }
            }
        }

        // verify 3x3 sub-box
        for i in 0..3 {
            for j in 0..3 {
                let mut set = HashSet::new();

                for row in 0..3 {
                    for column in 0..3 {
                        let ch = board[i * 3 + row][j * 3 + column];
                        if ch == '.' { continue }

                        if set.contains(&ch) {
                            return false
                        } else {
                            set.insert(ch);
                        }
                    }
                }
            }
        }
        
        true
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 0: One Pass.
pub struct Solution1;
impl Solution for Solution1 {

    fn is_valid_sudoku(&self, board: Vec<Vec<char>>) -> bool {

        use std::collections::HashSet;

        for i in 0..9 {
            let mut rows    = HashSet::new();
            let mut columns = HashSet::new();
            let mut subbox  = HashSet::new();

            for j in 0..9 {
                // verify rows
                let row_ch = board[i][j];
                if row_ch != '.' && rows.contains(&row_ch) {
                    return false
                } else {
                    rows.insert(row_ch);
                }
                
                // verify columns
                let column_ch = board[j][i];
                if column_ch != '.' && columns.contains(&column_ch) {
                    return false
                } else {
                    columns.insert(column_ch);
                }

                // verify sub-box
                let row    = 3 * (i / 3) + j / 3;
                let column = 3 * (i % 3) + j % 3;
                let subbox_ch = board[row][column];
                if subbox_ch != '.' && subbox.contains(&subbox_ch) {
                    return false
                } else {
                    subbox.insert(subbox_ch);
                }
            }
        }
       
        true
    }
}
// -----------------------------------------------------------------------------

