//!
//! Jump Game
//!
//! https://leetcode.com/problems/jump-game/
//!
//! Given an array of non-negative integers, you are initially positioned at the first index of the array.
//!
//! Each element in the array represents your maximum jump length at that position.
//!
//! Determine if you are able to reach the last index.
//!
//! **Example 1:**
//! ```text
//! Input: [2,3,1,1,4]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: [3,2,1,0,4]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what. Its maximum
//!              jump length is 0, which makes it impossible to reach the last index.
//! ```
//!



pub type Input  = Vec<i32>;
pub type Output = bool;

pub trait Solution {
    fn can_jump(&self, nums: Vec<i32>) -> bool;
}

// -----------------------------------------------------------------------------
/// Approach 0: Greedy Strategy.
pub struct Solution0;
impl Solution for Solution0 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {

        let mut location = 0;

        while location < nums.len() - 1 {
            let max_step = nums[location] as usize;
            let boundary = location + max_step;
            if boundary >= nums.len() - 1 { return true }

            let mut max_extra = 0;
            let mut max_i = 0;
            for i in 1..=max_step {
                if nums[location + i] + i as i32 >= max_extra {
                    max_extra = nums[location + i] + i as i32;
                    max_i = i;
                }
            }

            if max_i == 0 { return false }
            location += max_i;
        }

        true
    }
}
// -----------------------------------------------------------------------------

