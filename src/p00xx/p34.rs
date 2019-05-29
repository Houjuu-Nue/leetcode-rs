//!
//! Find First and Last Position of Element in Sorted Array
//!
//! https://leetcode.com/problems/find-first-and-last-position-of-element-in-sorted-array/
//!
//! Given an array of integers `nums` sorted in ascending order, find the starting and ending position of a given `target` value.
//!
//! Your algorithm's runtime complexity must be in the order of *O(log n)*.
//!
//! If the target is not found in the array, return `[-1, -1]`.
//!
//! **Example 1:**
//! ```text
//! Input: nums = [5,7,7,8,8,10], target = 8
//! Output: [3,4]
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: nums = [5,7,7,8,8,10], target = 6
//! Output: [-1,-1]
//! ```
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = Vec<i32>;

pub trait Solution {
    fn search_range(&self, nums: Vec<i32>, target: i32) -> Vec<i32>;
}

// -----------------------------------------------------------------------------
/// Approach 0: A cheat method based on binary search.
/// The runtime complexity is O(n), not O(log(n)).
pub struct Solution0;
impl Solution for Solution0 {

    fn search_range(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {

        if let Ok(location) = nums.binary_search(&target) {
            let mut left  = location;
            let mut right = location;

            while left > 0 { if nums[left - 1] == target { left -= 1 } else { break } }
            while right < nums.len() - 1 { if nums[right + 1] == target { right += 1 } else { break } }

            vec![left as i32, right as i32]
        } else {
            vec![-1, -1]
        }
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Linear Scan.
pub struct Solution1;
impl Solution for Solution1 {

    fn search_range(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {

        if let Some(left) = nums.iter().position(|&v| v == target) {
            let right = nums.iter().rev().position(|&v| v == target).unwrap();
            vec![left as i32, (nums.len() - right - 1) as i32]
        } else {
            vec![-1, -1]
        }
    }
}
// -----------------------------------------------------------------------------



// -----------------------------------------------------------------------------
/// Approach 2: Binary Seach.
pub struct Solution2;
impl Solution for Solution2 {

    fn search_range(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {

        if nums.is_empty() { return vec![-1, -1] }

        // search the left bound.
        let mut left  = 0;
        let mut right = nums.len();

        while left < right {
            let middle = (left + right) / 2;

            if nums[middle] < target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        if left == nums.len() || nums[left] != target { return vec![-1, -1] }
        let left_bound = left;

        // search the right bound.
        left = 0;
        right = nums.len();
        
        while left < right {
            let middle = (left + right) / 2;
            
            if nums[middle] <= target {
                left = middle + 1;
            } else {
                right = middle;
            }
        }

        let right_bound = right - 1;

        vec![left_bound as i32, right_bound as i32]
    }
}
// -----------------------------------------------------------------------------

