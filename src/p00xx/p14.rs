//!
//! Longest Common Prefix
//!
//! https://leetcode.com/problems/longest-common-prefix/
//! 
//! Write a function to find the longest common prefix string amongst an array of strings.
//! 
//! If there is no common prefix, return an empty string `""`.
//! 
//! ## Example 1:
//! ```text
//! Input: ["flower", "flow", "flight"]
//! Output: "fl"
//! ```
//! 
//! ## Example 2:
//! ```text
//! Input: ["dog", "racecar", "car"]
//! Output: ""
//! Explanation: There is no common prefix among the input strings.
//! ```
//!

pub type Input  = Vec<String>;
pub type Output = String;

pub trait Solution {
    fn longest_common_prefix(&self, strs: Vec<String>) -> String;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_common_prefix(&self, strs: Vec<String>) -> String {
        strs[0].clone()
    }
}
// -----------------------------------------------------------------------------
