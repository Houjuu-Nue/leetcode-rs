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
/// Approach 0: Vertical scanning.
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

        let mut prefix = if let Some(s) = strs.pop() {
            s
        } else {
            return String::new()
        };

        for s in strs {
            while s.starts_with(&prefix) == false {
                prefix.pop();

                if prefix.is_empty() {
                    return String::new()
                }
            }
        }
        
        prefix
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Divide and conquer.
pub struct Solution2;
impl Solution for Solution2 {

    fn longest_common_prefix(&self, strs: Vec<String>) -> String {

        if strs.is_empty() { return String::new() }

        let strs: Vec<_> = strs.into_iter()
            .map(|s| s.into_bytes())
            .collect();
        let prefix = longest_common_prefix_(&strs).to_vec();
        unsafe { String::from_utf8_unchecked(prefix) }
    }
}

fn longest_common_prefix_<'a, 'b>(strs: &'a [Vec<u8>]) -> &'b [u8]
    where 'a: 'b {

    let len = strs.len();

    if len == 1 {
        &strs[0]
    } else {
        let middle = strs.len() / 2;
        let s1 = longest_common_prefix_(&strs[0..middle]);
        let s2 = longest_common_prefix_(&strs[middle..len]);
        prefix(s1, s2)
    }
}

fn prefix<'a, 'b>(s1: &'a [u8], s2: &'a [u8]) -> &'b [u8]
    where 'a: 'b {
    
    use std::cmp::min;
    let min_len = min(s1.len(), s2.len());
    
    for i in 0..min_len {
        if s1[i] != s2[i] {
            return &s1[0..i]
        }
    }

    &s1[0..min_len]
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 3: Binary search.
pub struct Solution3;
impl Solution for Solution3 {

    fn longest_common_prefix(&self, strs: Vec<String>) -> String {

        if strs.is_empty() { return String::new() }
        
        let min_len = strs.iter()
            .map(|s| s.len())
            .min()
            .unwrap();
        
        let mut low = 1;
        let mut high = min_len;

        while low <= high {
            let middle = (low + high) / 2;
            if is_common_prefix(&strs, middle) {
                low = middle + 1;
            } else {
                high = middle - 1;
            }
        }

        let end = (low + high) / 2;
        strs[0][0..end].to_string()
    }
}

fn is_common_prefix(strs: &[String], middle: usize) -> bool {

    let prefix = unsafe {
        strs[0].get_unchecked(0..middle)
    };

    strs.iter()
        .all(|s| s.starts_with(prefix))
}
// -----------------------------------------------------------------------------
