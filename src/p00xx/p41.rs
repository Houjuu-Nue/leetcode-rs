//!
//! First Missing Positive
//!
//! https://leetcode.com/problems/first-missing-positive/
//!
//! Given an unsorted integer array, find the smallest missing positive integer.
//!
//! **Example 1:**
//! ```text
//! Input: [1,2,0]
//! Output: 3
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: [3,4,-1,1]
//! Output: 2
//! ```
//!
//! **Example 3:**
//! ```text
//! Input: [7,8,9,11,12]
//! Output: 1
//! ```
//!
//! **Note:**
//!
//! Your algorithm should run in O(n) time and uses constant extra space.
//!



pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn first_missing_positive(&self, nums: Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Position partial number.
pub struct Solution0;
impl Solution for Solution0 {

    fn first_missing_positive(&self, mut nums: Vec<i32>) -> i32 {

        let len = nums.len() as i32;
        
        for i in 0..nums.len() {
            loop {
                let num = nums[i];
                if num > 0 && num < len && num != i as i32 + 1 {
                    let swap_location = (num - 1) as usize;
                    if num != nums[swap_location] {
                        nums.swap(i, swap_location);
                        continue
                    }
                }

                break
            }
        }

        for (i, &num) in nums.iter().enumerate() {
            let i = (i + 1) as i32;
            if i != num {
                return i
            }
        }
        
        (nums.len() + 1) as i32
    }
}
// -----------------------------------------------------------------------------

