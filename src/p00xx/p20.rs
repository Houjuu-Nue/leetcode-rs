//!
//! Valid Parentheses
//!
//! https://leetcode.com/problems/valid-parentheses/
//!
//! Given a string containing just the characters `'('`, `')'`, `'{'`, `'}'`, `'['` and `']'`, determine if the input string is valid.
//!
//! An input string is valid if:
//!
//!    1. Open brackets must be closed by the same type of brackets.
//!
//!    2. Open brackets must be closed in the correct order.
//!
//! Note that an empty string is also considered valid.
//!
//! **Example 1:**
//! ```text
//! Input: "()"
//! Output: true
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: "()[]{}"
//! Output: true
//! ```
//!
//! **Example 3:**
//! ```text
//! Input: "(]"
//! Output: false
//! ```
//!
//! **Example 4:**
//! ```text
//! Input: "([)]"
//! Output: false
//! ```
//!
//! **Example 5:**
//! ```text
//! Input: "{[]}"
//! Output: true
//! ```
//!
//!


pub type Input  = String;
pub type Output = bool;

pub trait Solution {
    fn is_valid(&self, s: String) -> bool; 
}

// -----------------------------------------------------------------------------
/// Approach 0: Stack
pub struct Solution0;
impl Solution for Solution0 {

    fn is_valid(&self, s: String) -> bool {

        let mut stack = Vec::new();
        for ch in s.chars() {
            match ch {
                '(' => stack.push(ch),
                ')' => {
                    if let Some(pair) = stack.pop() {
                        if pair == '(' {
                            continue;
                        }
                    }
                    return false
                },
                '[' => stack.push(ch),
                ']' => {
                    if let Some(pair) = stack.pop() {
                        if pair == '[' {
                            continue
                        }
                    }
                    return false
                },
                '{' => stack.push(ch),
                '}' => {
                    if let Some(pair) = stack.pop() {
                        if pair == '{' {
                            continue
                        }
                    }
                    return false
                },
                _ => unreachable!()
            }
        }

        stack.is_empty()
    }
}
// -----------------------------------------------------------------------------

