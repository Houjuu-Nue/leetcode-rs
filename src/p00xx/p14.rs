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
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_common_prefix(&self, strs: Vec<String>) -> String {

        let mut strs: Vec<_> = strs.into_iter()
            .map(|s| s.into_bytes().into_iter())
            .collect();
        let mut test_str = if let Some(s) = strs.pop() { s } else { return String::new() };
        let mut result = String::new();

        while let Some(ch) = test_str.next() {
            for s in strs.iter_mut() {

                match s.next() {
                    | Some(c) if c != ch => return result,
                    | None => return result,
                    | _ => {},
                }
            }
            
            result.push(ch as char);
        }
        
        result
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Horizontal scanning.
pub struct Solution1;
impl Solution for Solution1 {

    fn longest_common_prefix(&self, mut strs: Vec<String>) -> String {

        let mut prefix = if let Some(s) = strs.pop() { s } else { return String::new() };

        for s in strs {
            while s.starts_with(&prefix) == false {
                prefix.pop();
                if prefix.is_empty() { return String::new() }
            }
        }
        
        prefix
    }
}
// -----------------------------------------------------------------------------
