//!
//! Divide Two Integers
//!
//! https://leetcode.com/problems/divide-two-integers/
//!
//! Given two integers **dividend** and **divisor**, divide two integers without using multiplication, division and mod operator.
//!
//! Return the quotient after dividing **dividend** by **divisor**.
//!
//! The integer division should truncate toward zero.
//!
//! **Example 1:**
//! ```text
//! Input: dividend = 10, divisor = 3
//! Output: 3
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: dividend = 7, divisor = -3
//! Output: -2
//! ```
//!
//! **Note:**
//! - Both dividend and divisor will be 32-bit signed integers.
//!
//! - The divisor will never be 0.
//!
//! - Assume we are dealing with an environment which could only store integers within the 32-bit signed integer range: [−2^31,  2^31 − 1].
//!
//! For the purpose of this problem, assume that your function returns 2^31 − 1 when the division result overflows.
//!
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub dividend: i32,
    pub divisor : i32,
}
pub type Output = i32;

pub trait Solution {

    fn divide(&self, dividend: i32, divisor: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Cheating.
pub struct Solution0;
impl Solution for Solution0 {

    fn divide(&self, dividend: i32, divisor: i32) -> i32 {

        match (dividend, divisor) {
            | (_, 1) => dividend,
            | (std::i32::MIN, -1) => std::i32::MAX,
            | (std::i32::MIN, std::i32::MIN) => 1,
            | (std::i32::MIN, _) => {
                let answer = dividend / -divisor;
                -answer
            },
            | (_, _) => {

                let sign = (dividend ^ divisor).signum();
                let answer = dividend.abs() / divisor.abs();

                answer * sign
            },
        }

    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Bit manipulations.
pub struct Solution1;
impl Solution for Solution1 {

    fn divide(&self, mut dividend: i32, divisor: i32) -> i32 {

        let mut answer = 0;
        
        // handle special case
        match (dividend, divisor) {
            | (std::i32::MIN, std::i32::MIN) => return 1,
            | (_, std::i32::MIN) => return 0,
            | (std::i32::MIN, -1) => return std::i32::MAX,
            | (std::i32::MIN, _) => {
                dividend += divisor.abs();
                answer += 1;
            },
            | _ => {},
        }

        // get sign of the result
        let sign = (dividend ^ divisor).signum();

        let mut dividend = dividend.abs() as i64;
        let divisor = divisor.abs() as i64;

        while dividend >= divisor {

            let mut i = 1;
            let mut tmp = divisor;
            while dividend >= tmp {
                dividend -= tmp;
                answer += i;
                i   <<= 1;
                tmp <<= 1;
            }
        }

        // handle the sign
        if sign == -1 {
            (!answer) + 1 // equaliant to `answer * -1`
        } else {
            answer
        }
    }
}
// -----------------------------------------------------------------------------

