//!
//! Regular Expression Matching
//!
//! https://leetcode.com/problems/regular-expression-matching/
//!
//! Given an input string (s) and a pattern (p), implement regular expression matching with support for '.' and '*'.
//!
//! ```text
//! '.' Matches any single character.
//! '*' Matches zero or more of the preceding element.
//! ```
//! The matching should cover the entire input string (not partial).
//! 
//! Note:
//! ```text
//! s could be empty and contains only lowercase letters a-z.
//! p could be empty and contains only lowercase letters a-z, and characters like . or *.
//! ```
//! 
//! ## Example 1:
//! ```text
//! Input:
//! s = "aa"
//! p = "a"
//! Output: false
//! Explanation: "a" does not match the entire string "aa".
//! ```
//!
//! ## Example 2:
//! ```text
//! Input:
//! s = "aa"
//! p = "a*"
//! Output: true
//! Explanation: '*' means zero or more of the precedeng element, 'a'.
//! Therefore, by repeating 'a' once, it becomes "aa".
//! ```
//! 
//! ## Example 3:
//! ```text
//! Input:
//! s = "ab"
//! p = ".*"
//! Output: true
//! Explanation: ".*" means "zero or more (*) of any character (.)".
//! ```
//! 
//! ## Example 4:
//! ```text
//! Input:
//! s = "aab"
//! p = "c*a*b"
//! Output: true
//! Explanation: c can be repeated 0 times, a can be repeated 1 time. Therefore it matches "aab".
//! ```
//! 
//! ## Example 5:
//! ```text
//! Input:
//! s = "mississippi"
//! p = "mis*is*p*."
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
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {
    fn is_match(&self, s: String, p: String) -> bool {
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_v0(&s, 0, &p, 0)
    }
}

fn is_match_v0(s: &[char], s_start: usize, p: &[char], p_start: usize) -> bool {

    match (s_start == s.len(), p_start == p.len()) {
        | (true,  true)  => return true,
        | (false, true)  => return false,
        | (true,  false) => {
            return match p[p_start] {
                | '*' => is_match_v0(s, s_start, p, p_start + 1),
                | _ => {
                    if p_start + 1 < p.len() && p[p_start + 1] == '*' {
                        is_match_v0(s, s_start, p, p_start + 2)
                    } else {
                        false
                    }
                },
            }
        },
        | (false, false) => {},
    }
    
    match p[p_start] {
        | '*' => {
            if s[s_start] == p[p_start - 1] || p[p_start - 1] == '.' {
                is_match_v0(s, s_start + 1, p, p_start) ||
                is_match_v0(s, s_start, p, p_start + 1)
            } else {
                is_match_v0(s, s_start, p, p_start + 1)
            }
        },
        | ch if ch == '.' || ch == s[s_start] => {
            is_match_v0(s, s_start + 1, p, p_start + 1) || {
                if p_start + 1 < p.len() && p[p_start + 1] == '*' {
                    is_match_v0(s, s_start, p, p_start + 2)
                } else {
                    false
                }
            }
        },
        | _ if p_start + 1 < p.len() && p[p_start + 1] == '*' => {
            is_match_v0(s, s_start, p, p_start + 2)
        },
        | _ => false,
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Recursion(Simpified version of Approach 0)
pub struct Solution1;
impl Solution for Solution1 {
    fn is_match(&self, s: String, p: String) -> bool {
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_v1(&s, 0, &p, 0)
    }
}

fn is_match_v1(s: &[char], s_start: usize, p: &[char], p_start: usize) -> bool {

    if p_start >= p.len() {
        return s_start == s.len()
    } else if s_start == s.len() {
        if p[p_start] == '*' || (p_start + 1 < p.len() && p[p_start + 1] == '*') {
            return is_match_v1(s, s_start, p, p_start + 1)
        } else {
            return false
        }
    }

    let s1 = s[s_start];
    let p1 = p[p_start];

    if p_start + 1 < p.len() &&  p[p_start + 1] == '*' {
        is_match_v1(s, s_start, p, p_start + 2) ||
        ((s1 == p1 || p1 == '.') && is_match_v1(s, s_start + 1, p, p_start))
    } else if p1 == '.' || s1 == p1 {
        is_match_v1(s, s_start + 1, p, p_start + 1)
    } else {
        false
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Dynamic Programming(Bottom-Up Variation)
pub struct Solution2;
impl Solution for Solution2 {
    fn is_match(&self, s: String, p: String) -> bool {
        
        if p.is_empty() { return s.is_empty() }
        
        let s: Vec<char> = s.chars().collect();
        let p: Vec<char> = p.chars().collect();

        is_match_v2(&s, &p)
    }
}

fn is_match_v2(s: &[char], p: &[char]) -> bool {

    let mut states = vec![vec![false; p.len() + 1]; s.len() + 1];
    states[s.len()][p.len()] = true;

    for i in (0..=s.len()).rev() {
        for j in (0..=(p.len() - 1)).rev() {

            let first_match = i < s.len() && (p[j] == s[i] || p[j] == '.');
            
            states[i][j] = if j + 1 < p.len() && p[j + 1] == '*' {
                states[i][j + 2] || (first_match && states[i + 1][j])
            } else {
                first_match && states[i + 1][j + 1]
            };
        }
    }

    states[0][0]
}
// -----------------------------------------------------------------------------
