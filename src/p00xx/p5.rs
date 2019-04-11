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

// Approach 0: Dynamic Programming(Beta) ----------------------------------------
pub struct Solution0;
impl Solution for Solution0 {

    fn longest_palindrome(&self, s: String) -> String {
        
        let bytes = s.into_bytes();
        if bytes.len() <= 1 { return String::from_utf8(bytes).unwrap() }

        let mut max_range = None;

        for i in 0..bytes.len() {

            let max_odd  = longest(&bytes, PairRange::new_odd(i));
            let max_even = longest(&bytes, PairRange::new_even(i, &bytes));

            let max_tmp = max_cmp(max_odd, max_even);
            max_range = max_cmp(max_tmp, max_range);
        }

        let max_range = max_range.unwrap();
        let sub_bytes = bytes.get(max_range.left..=max_range.right)
            .unwrap().iter().cloned().collect();
        String::from_utf8(sub_bytes).unwrap()
    }
}

#[derive(Debug, Clone, Copy)]
struct PairRange {
    left : usize,
    right: usize,
}

impl PairRange {

    fn new_odd(i: usize) -> Option<PairRange> {
        Some(PairRange { left: i, right: i })
    }
    
    fn new_even(i: usize, s: &[u8]) -> Option<PairRange> {
        if i + 1 < s.len() && s[i] == s[i + 1] {
            Some(PairRange { left: i, right: i + 1 })
        } else {
            None
        }
    }

    fn length(&self) -> usize {
        self.right - self.left + 1
    }

    fn extend(&self, s: &[u8]) -> Option<PairRange> {

        if self.left == 0 || self.right + 1 == s.len() { return None }
        if s[self.left - 1] != s[self.right + 1] { return None }

        let extend = PairRange {
            left : self.left  - 1,
            right: self.right + 1,
        };
        Some(extend)
    }
}

fn max_cmp(r1: Option<PairRange>, r2: Option<PairRange>) -> Option<PairRange> {

    if r1.is_none() { return r2 }
    if r2.is_none() { return r1 }

    if r1.unwrap().length() > r2.unwrap().length() {
        r1
    } else {
        r2
    }
}

fn longest(s: &[u8], range: Option<PairRange>) -> Option<PairRange> {

    let range = range?;

    if let Some(extend_range) = range.extend(s) {
        longest(s, Some(extend_range))
            .or(Some(range))
    } else {
        Some(range)
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
