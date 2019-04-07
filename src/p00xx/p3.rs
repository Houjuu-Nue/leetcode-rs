//!
//! https://leetcode.com/problems/longest-substring-without-repeating-characters/
//!
//! Given a string, find the length of the longest substring without repeating characters.
//! 
//! # Example 1:
//! ```
//! Input: "abcabcbb"
//! Output: 3 
//! Explanation: The answer is "abc", with the length of 3. 
//! ```
//! 
//! # Example 2:
//! ```
//! Input: "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//! ```
//!
//! # Example 3:
//! ```
//! Input: "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3. 
//! ```
//!
//! Note that the answer must be a substring, "pwke" is a subsequence and not a substring.

pub type Input = String;
pub type Answer = i32;

pub trait Solution {
    fn length_of_longest_substring(&self, s: String) -> i32;
}

pub struct Solution1;
impl Solution for Solution1 {

    fn length_of_longest_substring(&self, s: String) -> i32 {
        3
    }
}
