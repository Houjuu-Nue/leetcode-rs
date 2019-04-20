//!
//! Palindrome Number
//!
//! https://leetcode.com/problems/palindrome-number/
//!
//! Determine whether an integer is a palindrome.
//! An integer is a palindrome when it reads the same backward as forward.
//! 
//! ## Example 1:
//! ```ignore
//! Input: 121
//! Output: tru
//! ```
//! 
//! ## Example 2:
//! ```ignore
//! Input: -121
//! Output: false
//! Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//! ```
//! 
//! ## Example 3:
//! ```ignore
//! Input: 10
//! Output: false
//! Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//! ```
//!


pub type Input  = i32;
pub type Output = bool;

pub trait Solution {
    fn is_palindrome(&self, x: i32) -> bool;
}

// -----------------------------------------------------------------------------
// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn is_palindrome(&self, x: i32) -> bool {
        true
    }
}
// -----------------------------------------------------------------------------
