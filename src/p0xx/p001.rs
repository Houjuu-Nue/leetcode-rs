//!
//! https://leetcode-cn.com/problems/two-sum/
//!
//! Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! # Example:
//! ```ignore
//! Given nums = [2, 7, 11, 15], target = 9,
//!
//! Because nums[0] + nums[1] = 2 + 7 = 9,
//! return [0, 1].
//! ```
//!

pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}

pub type Answer = Vec<i32>;

pub trait Solution {
    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Answer;
}

// Approach 1: Brute Force
pub struct Solution1;
impl Solution for Solution1 {

    fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {

        for i in 0..(nums.len() - 1) {
            
            for j in (i + 1)..nums.len() {
                let result = nums[i] + nums[j];

                if result == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        Vec::new()
    }
}
