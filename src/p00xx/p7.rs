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
// Approach 0: Pop and Push Digits & Check before Overflow.
pub struct Solution0;
impl Solution for Solution0 {

    fn reverse(&self, x: i32) -> i32 {

        let is_minus = x < 0;
        let mut x = x.abs();

        let mut result: i32 = 0;

        while x > 0 {
            let remainder = x % 10;

            if let Some(mul_result) = result.checked_mul(10) {
                result = mul_result + remainder;
            } else {
                return 0
            }

            x /= 10;
        }

        if is_minus { result *= -1; }
        result
    }
}
// -----------------------------------------------------------------------------
