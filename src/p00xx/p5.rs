//!
//! Longest Palindromic Substring
//!
//! https://leetcode.com/problems/longest-palindromic-substring/
//!
//! Given a string s, find the longest palindromic substring in s. You may assume that the maximum length of s is 1000.
//! 
//! ## Example 1:
//! ```ignore
//! Input: "babad"
//! Output: "bab"
//! Note: "aba" is also a valid answer.
//! ``` 
//!
//! ## Example 2:
//! ```ignore
//! Input: "cbbd"
//! Output: "bb"
//! ```
//!

pub type Input  = String;
pub type Output = String;

pub trait Solution {
    fn longest_palindrome(&self, s: String) -> String;
}

pub struct Solution0;
impl Solution for Solution0 {

    fn longest_palindrome(&self, s: String) -> String {
        String::from("bad")
    }
}
