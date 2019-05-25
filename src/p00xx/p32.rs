//!
//! Longest Valid Parentheses
//!
//! https://leetcode.com/problems/longest-valid-parentheses/
//!
//! Given a string containing just the characters `'('` and `')'`, find the length of the longest valid (well-formed) parentheses substring.
//!
//! **Example 1:**
//! ```text
//! Input: "(()"
//! Output: 2
//! Explanation: The longest valid parentheses substring is "()"
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: ")()())"
//! Output: 4
//! Explanation: The longest valid parentheses substring is "()()"
//! ```
//!


pub type Input  = String;
pub type Output = i32;

pub trait Solution {
    fn longest_valid_parentheses(&self, s: String) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_valid_parentheses(&self, s: String) -> i32 {

        let mut max = 0;
        for i in 0..s.len() {
            for j in ((i + 2)..=s.len()).step_by(2) {

                let sub_string = &s[i..j];
                if is_valid(sub_string) {
                    dbg!(sub_string);
                    max = max.max(sub_string.len());
                }
            }
        }

        max as i32
    }
}

fn is_valid(s: &str) -> bool {

    let mut parenthes = 0;
    
    for ch in s.chars() {
        match ch {
            | '(' => parenthes += 1,
            | ')' => {
                if parenthes > 0 {
                    parenthes -= 1;
                } else {
                    return false
                }
            },
            | _ => unreachable!()
        }
    }

    parenthes == 0
}
// -----------------------------------------------------------------------------

