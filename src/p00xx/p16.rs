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
        
        let mut closest = nums[0] + nums[1] + nums[2];

        for i in 0..(nums.len() - 2) {
            for j in (i + 1)..(nums.len() - 1) {
                for k in (j + 1)..nums.len() {

                    let sum = nums[i] + nums[j] + nums[k];
                    if (sum - target).abs() < (closest - target).abs() {
                        closest = sum;
                    }
                }
            }
        }

        closest
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Triplet Pointers(similiar method to #P15).
pub struct Solution1;
impl Solution for Solution1 {

    fn three_sum_closest(&self, mut nums: Vec<i32>, target: i32) -> i32 {

        nums.sort_unstable();
        let mut min_distance = ::std::i32::MAX;
        let mut min_sum = ::std::i32::MAX;

        for start in 0..(nums.len() - 2) {
            
            if start > 0 && nums[start] == nums[start - 1] {
                continue
            }

            let mut middle = start + 1;
            let mut end = nums.len() - 1;

            while middle < end {
                let sum = nums[start] + nums[middle] + nums[end];
                let distance = (sum - target).abs();

                if distance == 0 {
                    return sum
                } else {
                    if distance < min_distance {
                        min_distance = distance;
                        min_sum = sum;
                    }

                    if sum - target > 0 {
                        end -= 1;
                    } else {
                        middle += 1;
                    }
                }
            }
        }
        
        min_sum
    }
}
// -----------------------------------------------------------------------------
