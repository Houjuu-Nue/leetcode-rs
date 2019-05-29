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

        if let Some(location) = binary_search(&nums, target) {
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

fn binary_search(nums: &Vec<i32>, target: i32) -> Option<usize> {
    
    if nums.is_empty() { return None; }
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left <= right && right != std::usize::MAX {

        let middle = (left + right) / 2;
        if nums[middle] == target { return Some(middle) }
        if nums[left]   == target { return Some(left) }
        if nums[right]  == target { return Some(right) }

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

    None
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

