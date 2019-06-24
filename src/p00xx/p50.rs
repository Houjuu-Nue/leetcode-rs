//!
//! Pow(x, n)
//!
//! https://leetcode.com/problems/powx-n/
//!
//! Implement [pow(x, n)](http://www.cplusplus.com/reference/valarray/pow/), which calculates x raised to the power n (x^n).
//!
//! **Example 1:**
//! ```text
//! Input: 2.00000, 10
//! Output: 1024.00000
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: 2.10000, 3
//! Output: 9.26100
//! ```
//!
//! **Example 3:**
//! ```text
//! Input: 2.00000, -2
//! Output: 0.25000
//! Explanation: 2-2 = 1/22 = 1/4 = 0.25
//! ```
//!
//! **Note:**
//! - -100.0 < x < 100.0
//! - n is a 32-bit signed integer, within the range [−2^31, 2^31 − 1].
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub x: f64,
    pub n: i32,
}
pub type Output = f64;

pub trait Solution {
    fn my_pow(&self, x: f64, n: i32) -> f64;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn my_pow(&self, x: f64, n: i32) -> f64 {

        if n == 0 { return 1.0 }
        if n > 0 {
            let mut product = 1.0;
            for _ in 1..=(n as usize) {
                product *= x;
            }
            product - product % 0.000001
        } else { // n < 0
            let mut product = 1.0;
            for _ in 1..=(-n as usize) {
                product /= x;
            }
            product - product % 0.000001
        }
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 1: Recursion with fast powering.
pub struct Solution1;
impl Solution for Solution1 {

    fn my_pow(&self, x: f64, n: i32) -> f64 {
        let ans = fast_powering(x, n);
        ans - ans % 0.000001
    }
}

fn fast_powering(x: f64, n: i32) -> f64 {

    if n == 0 { return 1.0 }

    let half = fast_powering(x, n / 2);

    if n % 2 == 0 { return half * half }

    if n > 0 {
        half * half * x
    } else {
        half * half / x
    }
}
// -----------------------------------------------------------------------------


