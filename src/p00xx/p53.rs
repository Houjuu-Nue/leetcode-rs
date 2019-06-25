//!
//! Maximum Subarray
//!
//! https://leetcode.com/problems/maximum-subarray/
//!
//! Given an integer array `nums`, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.
//!
//! **Example:**
//! ```text
//! Input: [-2,1,-3,4,-1,2,1,-5,4],
//! Output: 6
//! Explanation: [4,-1,2,1] has the largest sum = 6.
//! ```
//!
//! **Follow up**:
//!
//! If you have figured out the O(n) solution, try coding another solution using the divide and conquer approach, which is more subtle.
//!



pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn max_sub_array(&self, nums: Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn max_sub_array(&self, nums: Vec<i32>) -> i32 {

        let mut max_sum = nums[0];

        for i in 0..nums.len() {
            for j in (i + 1)..=nums.len() {

                let sum: i32 = nums[i..j].iter().sum();
                max_sum = sum.max(max_sum);
            }
        }

        max_sum
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 1: Dynamic Programming - Kadane Algorithm.
pub struct Solution1;
impl Solution for Solution1 {

    fn max_sub_array(&self, nums: Vec<i32>) -> i32 {

        let mut max_sum = nums[0];
        let mut sum = 0;
        
        for num in nums {
            if sum > 0 {
                sum += num;
            } else {
                sum = num;
            }
            
            max_sum = sum.max(max_sum);
        }

        max_sum
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 2: Improved Brute Force.
pub struct Solution2;
impl Solution for Solution2 {

    fn max_sub_array(&self, nums: Vec<i32>) -> i32 {

        let mut max_sum = nums[0];

        for i in 0..nums.len() {
            let mut sum = 0;
            for j in i..nums.len() {

                sum += nums[j];
                max_sum = sum.max(max_sum);
            }
        }

        max_sum
    }
}
// -----------------------------------------------------------------------------

