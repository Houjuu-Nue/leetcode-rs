//!
//! 4Sum
//!
//! https://leetcode.com/problems/4sum/
//!
//! Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
//!
//! **Note:**
//! The solution set must not contain duplicate quadruplets.
//!
//! **Example:**
//! ```text
//! Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
//!
//! A solution set is:.
//!
//! [.
//!
//!  [-1,  0, 0, 1],.
//!
//!  [-2, -1, 1, 2],.
//!
//!  [-2,  0, 0, 2].
//!
//! ].
//! ```
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn four_sum(&self, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn four_sum(&self, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        Vec::new()
    }
}
// -----------------------------------------------------------------------------

