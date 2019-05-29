//!
//! Search Insert Position
//!
//! https://leetcode.com/problems/search-insert-position/<Paste>
//!
//! Given a sorted array and a target value, return the index if the target is found.
//!
//! If not, return the index where it would be if it were inserted in order.
//!
//! You may assume no duplicates in the array.
//!
//! **Example 1:**
//! ```text
//! Input: [1,3,5,6], 5
//! Output: 2
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: [1,3,5,6], 2
//! Output: 1
//! ```
//!
//! **Example 3:**
//! ```text
//! Input: [1,3,5,6], 7
//! Output: 4
//! ```
//!
//! **Example 4:**
//! ```text
//! Input: [1,3,5,6], 0
//! Output: 0
//! ```
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = i32;

pub trait Solution {
    fn search_insert(&self, nums: Vec<i32>, target: i32) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Build-in method.
pub struct Solution0;
impl Solution for Solution0 {

    fn search_insert(&self, nums: Vec<i32>, target: i32) -> i32 {

        match nums.binary_search(&target) {
            | Ok(index) => index as i32,
            | Err(insert_location) => insert_location as i32
        }

    }
}
// -----------------------------------------------------------------------------

