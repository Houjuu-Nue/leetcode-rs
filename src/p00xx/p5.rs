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

// Approach 0: Dynamic Programming ---------------------------------------------
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_palindrome(&self, s: String) -> String {
        
        let bytes = s.into_bytes();
        let len = bytes.len();
        if len <= 1 { return String::from_utf8(bytes).unwrap() }

        // p store the result.
        let mut p: Vec<Vec<bool>> = vec![vec![false; len]; len];
        
        // Initialize boundary.
        for i in 0..len {
            p[i][i] = true;
        }
        for i in 0..(len - 1) {
            p[i][i + 1] = bytes[i] == bytes[i + 1];
        }

        // Start to search
        let mut max_length = 1;
        let mut max_start  = 0;

        for j in 1..len {
            for i in 0..j {

                if j - i != 1 {
                    p[i][j] = p[i + 1][j - 1] && bytes[i] == bytes[j];
                }

                if p[i][j] == true {
                    let length = j - i + 1;
                    if length > max_length {
                        max_start = i;
                        max_length = length;
                    }
                }
            }
        }
        
        unsafe {
            let sub_bytes = bytes.get_unchecked(max_start..(max_start + max_length))
                .iter().cloned().collect();
            String::from_utf8_unchecked(sub_bytes)
        }

    }
}

// -----------------------------------------------------------------------------


// Approach 1: Brute Force -----------------------------------------------------
pub struct Solution1;
impl Solution for Solution1 {

    fn longest_palindrome(&self, s: String) -> String {

        let bytes = s.into_bytes();
        if bytes.len() <= 1 { return String::from_utf8(bytes).unwrap() }
        
        let mut max_length = 0;
        let mut max_left   = 0;
        let mut max_right  = 0;
        
        for i in 0..bytes.len() {
            for j in i..bytes.len() {

                if is_palindrome(&bytes, i, j) {
                    let length = j - i + 1;
                    if length > max_length {
           
                        max_length = length;
                        max_left  = i;
                        max_right = j;
                    }
                }
            }
        }

        let sub_bytes = bytes.get(max_left..=max_right)
            .unwrap().iter().cloned().collect();
        String::from_utf8(sub_bytes).unwrap()
    }
}

fn is_palindrome(s: &[u8], mut left: usize, mut right: usize) -> bool {

    while left < right {
        if s[left] != s[right] { return false }

        left += 1;
        if let Some(new_right) = right.checked_sub(1) {
            right = new_right;
        } else {
            return false
        }
    }

    true
}
// -----------------------------------------------------------------------------
