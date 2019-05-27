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


// -----------------------------------------------------------------------------
/// Approach 1: Dynamic Programming.
pub struct Solution1;
impl Solution for Solution1 {

    fn longest_valid_parentheses(&self, s: String) -> i32 {

        let s: Vec<char> = s.chars().collect();
        let mut max = 0;
        let mut dp = vec![0; s.len()];

        for i in 1..s.len() {
            if s[i] == ')' {
                if s[i - 1] == '(' {
                    if i > 1 {
                        dp[i] = dp[i - 2] + 2;
                    } else {
                        dp[i] = 2;
                    }
                } else if i - dp[i - 1] > 0 && s[i - dp[i - 1] - 1] == '(' {
                    if i - dp[i - 1] > 1 {
                        dp[i] = dp[i - 1] + dp[i - dp[i - 1] - 2] + 2;
                    } else {
                        dp[i] = dp[i - 1] + 2;
                    }
                }

                max = max.max(dp[i]);
            }
        }

        max as i32
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Stack.
pub struct Solution2;
impl Solution for Solution2 {

    fn longest_valid_parentheses(&self, s: String) -> i32 {

        let mut max = 0;
        let mut stack = Vec::new();
        stack.push(-1);

        for (i, ch) in s.chars().enumerate() {
            let i = i as i32;

            match ch {
                | '(' => stack.push(i),
                | ')' => {
                    stack.pop();
                    if let Some(last_invalid) = stack.last().cloned() {
                        max = max.max((i - last_invalid) as usize);
                    } else {
                        stack.push(i);
                    }
                },
                | _ => unreachable!()
            }
        }

        max as i32
    }
}
// -----------------------------------------------------------------------------

