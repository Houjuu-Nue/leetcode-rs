//!
//! Wildcard Matching
//!
//! https://leetcode.com/problems/wildcard-matching/
//!
//! Given an input string (`s`) and a pattern (`p`), implement wildcard pattern matching with support for `'?'` and `'*'`.
//!
//! ```text
//! '?' Matches any single character.
//! '*' Matches any sequence of characters (including the empty sequence).
//! ```
//!
//! The matching should cover the **entire** input string (not partial).
//!
//! **Note:**
//!
//! - `s` could be empty and contains only lowercase letters `a-z`.
//!
//! - `p` could be empty and contains only lowercase letters `a-z`, and characters like `?` or `*`.
//!
//! **Example 1:**
//! ```text
//! Input:
//! s = "aa"
//! p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//! ```
//!
//! **Example 2:**
//! ```text
//! Input:
//! s = "aa"
//! p = "*"
//! Output: true
//! Explanation: '*' matches any sequence.
//! ```
//!
//! **Example 3:**
//! ```text
//! Input:
//! s = "cb"
//! p = "?a"
//! Output: false
//! Explanation: '?' matches 'c', but the second letter is 'a', which does not match 'b'.
//! ```
//!
//! **Example 4:**
//! ```text
//! Input:
//! s = "adceb"
//! p = "*a*b"
//! Output: true
//! Explanation: The first '*' matches the empty sequence, while the second '*' matches the substring "dce".
//! ```
//!
//! **Example 5:**
//! ```text
//! Input:
//! s = "acdcb"
//! p = "a*c?b"
//! Output: false
//! ```
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub s: String,
    pub p: String,
}
pub type Output = bool;

pub trait Solution {
    fn is_match(&self, s: String, p: String) -> bool;
}

// -----------------------------------------------------------------------------
/// Approach 0: Recursive Search.
pub struct Solution0;
impl Solution for Solution0 {

    fn is_match(&self, s: String, p: String) -> bool {
        
        if p.is_empty() { return s.is_empty() }
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_(&s, &p, s.len(), p.len())
    }
}

fn is_match_(s: &[char], p: &[char], s_len: usize, p_len: usize) -> bool {

    match (s_len == 0, p_len == 0) {
        | (true, true)  => return true,
        | (true, false) => return p[0..p_len].iter().all(|&ch| ch == '*'),
        | (false, true) => return s[0..s_len].iter().all(|&ch| ch == '*'),
        | _ => {},
    }

    let ch_p = p[p_len - 1];
    // dbg!(&s[0..s_len].iter().collect::<String>());
    // dbg!(&p[0..p_len].iter().collect::<String>());

    match ch_p {
        | '*' => is_match_(s, p, s_len - 1, p_len) || is_match_(s, p, s_len, p_len - 1),
        | '?' => is_match_(s, p, s_len - 1, p_len - 1),
        | ch_p if s[s_len - 1] == ch_p => is_match_(s, p, s_len - 1, p_len - 1),
        | _ => false
    }
}
// -----------------------------------------------------------------------------

