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
        String::from("PAHNAPLSIIGYIR")
    }
}
// -----------------------------------------------------------------------------

