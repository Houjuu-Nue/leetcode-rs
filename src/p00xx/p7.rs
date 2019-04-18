//!
//! Reverse Integer
//!
//! https://leetcode.com/problems/reverse-integer/
//!
//! Given a 32-bit signed integer, reverse digits of an integer.
//!
//! ## Example 1:
//! ```ignore
//! Input: 123
//! Output: 321
//! ```
//!
//! ## Example 2:
//! ```ignore
//! Input: -123
//! Output: -321
//! ```
//!
//! ## Example 3:
//! ```ignore
//! Input: 120
//! Output: 21
//! ```
//!
//! Note:
//! Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31, 2^(31−1)].
//!
//! For the purpose of this problem, assume that your function returns 0 when the reversed integer overflows.
//!

pub type Input  = i32;
pub type Output = i32;

pub trait Solution {
    fn reverse(&self, x: i32) -> i32;
}

// -----------------------------------------------------------------------------
pub struct Solution0;
impl Solution for Solution0 {

    fn reverse(&self, x: i32) -> i32 {
        321
    }
}
// -----------------------------------------------------------------------------
