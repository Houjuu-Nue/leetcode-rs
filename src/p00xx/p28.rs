//!
//! Implement strStr()
//!
//! https://leetcode.com/problems/implement-strstr/
//!
//! Implement [strStr()](http://www.cplusplus.com/reference/cstring/strstr/).
//!
//! Return the index of the first occurrence of needle in haystack, or **-1** if needle is not part of haystack.
//!
//! **Example 1:**
//! ```text
//! Input: haystack = "hello", needle = "ll"
//! Output: 2
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: haystack = "aaaaa", needle = "bba"
//! Output: -1
//! ```
//!
//! **Clarification:**
//!
//! What should we return when `needle` is an empty string?
//!
//! This is a great question to ask during an interview.
//!
//! For the purpose of this problem, we will return 0 when `needle` is an empty string.
//!
//! This is consistent to C's [strstr()](http://www.cplusplus.com/reference/cstring/strstr/) and Java's [indexOf()](https://docs.oracle.com/javase/7/docs/api/java/lang/String.html#indexOf(java.lang.String)).
//!
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub haystack: String, 
    pub needle  : String,
}
pub type Output = i32;

pub trait Solution {
	fn str_str(&self, haystack: String, needle: String) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Rust build-in method.
pub struct Solution0;
impl Solution for Solution0 {

	fn str_str(&self, haystack: String, needle: String) -> i32 {

        haystack.find(&needle)
            .map(|first| first as i32)
            .unwrap_or(-1)
	}
}
// -----------------------------------------------------------------------------

