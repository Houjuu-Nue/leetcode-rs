//!
//! Search in Rotated Sorted Array
//!
//! https://leetcode.com/problems/search-in-rotated-sorted-array/
//!
//! Suppose an array sorted in ascending order is rotated at some pivot unknown to you beforehand. (i.e., `[0,1,2,4,5,6,7]` might become `[4,5,6,7,0,1,2])`.
//!
//! You are given a target value to search.
//!
//! If found in the array return its index, otherwise return `-1`.
//!
//! You may assume no duplicate exists in the array.
//!
//! Your algorithm's runtime complexity must be in the order of O(log n).
//!
//! **Example 1:**
//! ```text
//! Input: nums = [4,5,6,7,0,1,2], target = 0
//! Output: 4
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: nums = [4,5,6,7,0,1,2], target = 3
//! Output: -1
//! ```
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = i32;

pub trait Solution {
    fn search(&self, nums: Vec<i32>, target: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Binary Search.
pub struct Solution0;
impl Solution for Solution0 {

    fn search(&self, nums: Vec<i32>, target: i32) -> i32 {

        binary_search(&nums, target)
            .map(|location| location as i32)
            .unwrap_or(-1)
    }
}

fn binary_search(nums: &[i32], target: i32) -> Option<usize> {

    if nums.is_empty() { return None }

    let left = 0;
    if nums[left] == target { return Some(left) }

    let right = nums.len() - 1;
    if nums[right] == target { return Some(right) }

    let middle = nums.len() / 2;
    if nums[middle] == target { return Some(middle) }

    if nums[0] < nums[middle] {
        // nums[left..=middle] is sorted
        if target > nums[left] && target < nums[middle] {
            // if target is in left sorted part
            binary_search(&nums[0..middle], target)
        } else {
            // search another right unsorted part
            binary_search(&nums[(middle + 1)..nums.len()], target)
                .and_then(|location| Some(location + middle + 1))
        }
    } else {
        // nums[middle..right] is sorted
        if target > nums[middle] && target < nums[right] {
            // if target is in the right sorted part
            binary_search(&nums[(middle + 1)..nums.len()], target)
                .and_then(|location| Some(location + middle + 1))
        } else {
            // search another left unsorted part
            binary_search(&nums[0..middle], target)
        }
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Simplify Binary Search.
pub struct Solution1;
impl Solution for Solution1 {

    fn search(&self, nums: Vec<i32>, target: i32) -> i32 {
        
        if nums.is_empty() { return 0; }
        let mut left = 0;
        let mut right = nums.len() - 1;

        while left <= right && right != std::usize::MAX {

            let middle = (left + right) / 2;
            if nums[middle] == target { return middle as i32 }
            if nums[left]   == target { return left as i32 }
            if nums[right]  == target { return right as i32 }

            if nums[left] < nums[middle] {
                if nums[left] <= target && target < nums[middle] {
                    right = middle - 1;
                } else {
                    left = middle + 1;
                }
            } else {
                if nums[middle] < target && target <= nums[right] {
                    left = middle + 1;
                } else {
                    right = middle - 1;
                }
            }
        }

        -1
    }
}
// -----------------------------------------------------------------------------

