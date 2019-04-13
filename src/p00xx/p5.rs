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

// Approach 0: Dynamic Programming Version 1 ----------------------------------
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_palindrome(&self, s: String) -> String {
        
        let bytes = s.into_bytes();
        let len = bytes.len();
        if len <= 1 { return unsafe { String::from_utf8_unchecked(bytes) } }

        // p store the result.
        let mut p: Vec<Vec<bool>> = vec![vec![false; len]; len];

        let mut max_length = 1;
        let mut max_start  = 0;

        // Start to search
        for j in 1..len {

            // Initialize boundary.
            p[j][j] = true;

            for i in 0..j {

                p[i][j] = if j - i != 1 {
                    p[i + 1][j - 1] && bytes[i] == bytes[j]
                } else {
                    bytes[i] == bytes[j]
                };

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

// Approach 1: Dynamic Programming Version 2 -----------------------------------
pub struct Solution1;
impl Solution for Solution1 {

    fn longest_palindrome(&self, s: String) -> String {

        let bytes = s.into_bytes();
        let len = bytes.len();
        if len <= 1 { return unsafe { String::from_utf8_unchecked(bytes) } }

        let mut dp: Vec<Vec<bool>> = vec![vec![false; len]; len];
        let mut left  = 0;
        let mut right = 0;

        let mut line = len - 1;
        while let Some(i) = line.checked_sub(1) {
            dp[i][i] = true;

            for j in (i + 1)..len {
                dp[i][j] = bytes[i] == bytes[j] && (j - i < 3 || dp[i + 1][j - 1]);

                if dp[i][j] && right - left <= j - i {
                    left  = i;
                    right = j;
                }
            }

            line = i;
        }

        unsafe {
            let sub_bytes = bytes.get_unchecked(left..=right)
                .iter().cloned().collect();
            String::from_utf8_unchecked(sub_bytes)
        }
    }
}
// -----------------------------------------------------------------------------


// Approach 2: Brute Force -----------------------------------------------------
pub struct Solution2;
impl Solution for Solution2 {

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

// -----------------------------------------------------------------------------
// Approach 3: Expand Around Center
pub struct Solution3;
impl Solution for Solution3 {

    fn longest_palindrome(&self, s: String) -> String {

        let bytes = s.into_bytes();
        if bytes.len() <= 1 { return String::from_utf8(bytes).unwrap() }

        let mut max_range = MaxRange { start: 0, length: 0 };

        for i in 0..bytes.len() {

            let max_odd  = expend_odd(&bytes, i);
            let max_even = expend_even(&bytes, i);

            let max_tmp = if max_odd.length > max_even.length { max_odd } else { max_even };

            if max_tmp.length > max_range.length {
                max_range = max_tmp;
            }
        }

        let sub_bytes = bytes.get(max_range.start..(max_range.start + max_range.length))
            .unwrap().iter().cloned().collect();
        String::from_utf8(sub_bytes).unwrap()
    }
}

struct MaxRange {
    start : usize,
    length: usize,
}

fn expend_odd(s: &[u8], center: usize) -> MaxRange {

    let len = s.len() as i32;
    let mut left  = (center as i32) - 1;
    let mut right = center as i32 + 1;
    let mut length = 1;
    let mut start = center;

    while left >= 0 && right < len && s[left as usize] == s[right as usize] {
        length += 2;
        start -= 1;

        left  -= 1;
        right += 1;
    }

    MaxRange { start, length }
}

fn expend_even(s: &[u8], left: usize) -> MaxRange {

    let mut length = 0;
    let mut start = left + 1;

    let len = s.len() as i32;
    let mut left  = left as i32;
    let mut right = left + 1;

    while left >= 0 && right < len && s[left as usize] == s[right as usize] {
        length += 2;
        start -= 1;

        left  -= 1;
        right += 1;
    }

    MaxRange { start, length }
}
// -----------------------------------------------------------------------------
