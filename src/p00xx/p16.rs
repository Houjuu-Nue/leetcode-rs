//!
//! 3Sum Closest
//!
//! https://leetcode.com/problems/3sum-closest/
//! 
//! Given an array nums of n integers and an integer target, find three integers in nums such that the sum is closest to target.
//!
//! Return the sum of the three integers.
//!
//! You may assume that each input would have exactly one solution.
//!
//! ## Example:
//! ```text
//! Given array nums = [-1, 2, 1, -4], and target = 1.
//! 
//! The sum that is closest to the target is 2. (-1 + 2 + 1 = 2).
//! ```
//!

#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = i32;

pub trait Solution {
    fn three_sum_closest(&self, nums: Vec<i32>, target: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn three_sum_closest(&self, nums: Vec<i32>, target: i32) -> i32 {
        2
    }
}
// -----------------------------------------------------------------------------
