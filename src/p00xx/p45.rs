//!
//! Jump Game II
//!
//! https://leetcode.com/problems/jump-game-ii/
//!
//! Given an array of non-negative integers, you are initially positioned at the first index of the array.
//!
//! Each element in the array represents your maximum jump length at that position.
//!
//! Your goal is to reach the last index in the minimum number of jumps.
//!
//! **Example:**
//! ```text
//! Input: [2,3,1,1,4]
//! Output: 2
//! Explanation: The minimum number of jumps to reach the last index is 2.
//!     Jump 1 step from index 0 to 1, then 3 steps to the last index.
//! ```
//!
//! **Note:**
//!
//! You can assume that you can always reach the last index.
//!



pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn jump(&self, nums: Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Greedy Strategy.
pub struct Solution0;
impl Solution for Solution0 {

    fn jump(&self, nums: Vec<i32>) -> i32 {

        let mut location = 0;
        let mut steps = 0;

        while location < nums.len() - 1 {
            let max_step = nums[location] as usize;
            let boundary = location + max_step;
            if boundary >= nums.len() - 1 { return steps + 1 }

            let mut max_extra = 0;
            let mut max_i = 1;
            for i in 1..=max_step {
                if nums[location + i] + i as i32 >= max_extra {
                    max_extra = nums[location + i] + i as i32;
                    max_i = i;
                }
            }

            location += max_i;
            steps += 1;
        }

        steps
    }
}
// -----------------------------------------------------------------------------

