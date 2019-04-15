//!
//! ZigZag Conversion
//!
//! https://leetcode.com/problems/zigzag-conversion/
//!
//! The string `"PAYPALISHIRING"` is written in a zigzag pattern on a given number of rows like this: (you may want to display this pattern in a fixed font for better legibility)
//! 
//! ```ignore
//! P   A   H   N
//! A P L S I I G
//! Y   I   R
//! ```
//! 
//! And then read line by line: `"PAHNAPLSIIGYIR"`
//! 
//! Write the code that will take a string and make this conversion given a number of rows:
//! 
//! string convert(string s, int numRows);
//! 
//! ## Example 1:
//! ```ignore
//! Input: s = "PAYPALISHIRING", numRows = 3
//! Output: "PAHNAPLSIIGYIR"
//! ```
//! 
//! ## Example 2:
//! ```ignore
//! Input: s = "PAYPALISHIRING", numRows = 4
//! Output: "PINALSIGYAHRPI"
//! Explanation:
//! 
//! P     I    N
//! A   L S  I G
//! Y A   H R
//! P     I
//! ```
//!

#[derive(Debug, Clone)]
pub struct Input {
    pub s: String,
    pub num_rows: i32,
}
pub type Output = String;

pub trait Solution {
    fn convert(&self, s: String, num_rows: i32) -> String;
}

// -----------------------------------------------------------------------------
pub struct Solution0;
impl Solution for Solution0 {

    fn convert(&self, s: String, num_rows: i32) -> String {

        let mut matrix = fill_matrix(s, num_rows as usize);
        matrix.sort_unstable_by(|ch1, ch2| {
            ch1.row.cmp(&ch2.row)
                .then_with(|| ch1.column.cmp(&ch2.column))
        });

        matrix.into_iter().map(|character| character.ch).collect()
    }
}

#[derive(Debug, Clone)]
struct Character {
    ch: char,
    row: usize,
    column: usize,
}

// input character into matrix.
fn fill_matrix(s: String, num_rows: usize) -> Vec<Character> {

    let chars: Vec<char> = s.chars().collect();

    // character count for each z loop.
    let z_loop_count;
    // column count for each z loop.
    let z_loop_column;

    if num_rows > 1 {
        z_loop_count = num_rows + num_rows - 2;
        z_loop_column = num_rows + 1 - 2;
    } else {
        z_loop_count = 1;
        z_loop_column = 1;
    }

    chars.into_iter().enumerate()
        .map(|(i, ch)| {

            let loop_count = i / z_loop_count;
            let mut column = loop_count * z_loop_column;
            let row;

            let leave_chars_count = i % z_loop_count + 1;
            if leave_chars_count <= num_rows {
                row = leave_chars_count - 1;
            } else {
                column += leave_chars_count - num_rows;
                row = z_loop_column - (column % z_loop_column);
            }

            Character { ch, row, column }

        }).collect()
}
// -----------------------------------------------------------------------------


// Approach 1: Sort by Row -----------------------------------------------------
pub struct Solution1;
impl Solution for Solution1 {

    fn convert(&self, s: String, num_rows: i32) -> String {

        use std::cmp::min;

        let num_rows = num_rows as usize;
        if num_rows == 1 { return s }

        let mut rows: Vec<String> = (0..min(num_rows, s.len()))
            .into_iter()
            .map(|_| String::new())
            .collect();

        enum Direction {
            Up(usize),
            Down(usize),
        }

        let mut dire = Direction::Down(0);

        for ch in s.chars() {
            match dire {
                | Direction::Down(row_index) => {

                    rows[row_index].push(ch);

                    dire = if row_index == num_rows - 1 {
                        Direction::Up(row_index - 1)
                    } else {
                        Direction::Down(row_index + 1)
                    };
                },
                | Direction::Up(row_index) => {

                    rows[row_index].push(ch);
                    
                    dire = if row_index == 0 {
                        Direction::Down(row_index + 1)
                    } else {
                        Direction::Up(row_index - 1)
                    };
                }
            }
        }

        rows.into_iter()
            .fold(String::new(), |acc, row| acc + &row)
    }
}
// -----------------------------------------------------------------------------
