//!
//! 3Sum
//!
//! https://leetcode.com/problems/3sum/
//! 
//! Given an array nums of n integers, are there elements a, b, c in nums such that a + b + c = 0?
//!
//! Find all unique triplets in the array which gives the sum of zero.
//!
//! **Note**:
//!
//! The solution set must not contain duplicate triplets.
//!
//! ## Example:
//! ```text
//! Given array nums = [-1, 0, 1, 2, -1, -4]
//!
//! A solution set is:
//! [
//!   [-1, 0, 1],
//!   [-1, -1, 2]
//! ]
//! ```
//!

pub type Input  = Vec<i32>;
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        Vec::new()
    }
}
// -----------------------------------------------------------------------------
