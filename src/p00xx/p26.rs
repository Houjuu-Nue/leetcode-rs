//!
//! Remove Duplicates from Sorted Array
//!
//! https://leetcode.com/problems/remove-duplicates-from-sorted-array/
//!
//! Given a sorted array nums, remove the duplicates [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) such that each element appear only once and return the new length.
//!
//! Do not allocate extra space for another array, you must do this by **modifying the input array** [in-place](https://en.wikipedia.org/wiki/In-place_algorithm) with O(1) extra memory.
//!
//! **Example 1:**
//! ```text
//! Given nums = [1,1,2],
//!
//! Your function should return length = 2, with the first two elements of nums being 1 and 2 respectively.
//!
//! It doesn't matter what you leave beyond the returned length.
//! ```
//!
//! **Example 2:**
//! ```text
//! Given nums = [0,0,1,1,1,2,2,3,3,4],
//!
//! Your function should return length = 5, with the first five elements of nums being modified to 0, 1, 2, 3, and 4 respectively.
//!
//! It doesn't matter what values are set beyond the returned length.
//! ```
//!
//! **Clarification**:
//!
//! Confused why the returned value is an integer but your answer is an array?
//!
//! Note that the input array is passed in by **reference**, which means modification to the input array will be known to the caller as well.
//! Internally you can think of this:
//!
//! ```text
//! // nums is passed in by reference. (i.e., without making a copy).
//! int len = removeDuplicates(nums);
//!
//! // any modification to nums in your function would be known by the caller.
//! // using the length returned by your function, it prints the first len elements.
//! for (int i = 0; i < len; i++) {
//!    print(nums[i]);
//! }
//! ```
//!


pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn remove_duplicates(&self, nums: &mut Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Two Pointers.
pub struct Solution0;
impl Solution for Solution0 {

    fn remove_duplicates(&self, nums: &mut Vec<i32>) -> i32 {

        if nums.is_empty() {
            return 0
        }

        let mut current = nums[0];
        let mut index = 0;
        let mut truncate = 0;

        for i in 1..nums.len() {
            if nums[i] != current {
                current = nums[i];
                index += 1;
                nums[index] = nums[i];
            } else {
                truncate += 1;
            }
        }

        nums.truncate(nums.len() - truncate);
        nums.len() as i32
    }
}
// -----------------------------------------------------------------------------

