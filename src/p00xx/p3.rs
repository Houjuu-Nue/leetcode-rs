//!
//! Longest Substring Without Repeating Characters
//!
//! https://leetcode.com/problems/longest-substring-without-repeating-characters/
//!
//! Given a string, find the length of the longest substring without repeating characters.
//! 
//! ## Example 1:
//! ```text
//! Input: "abcabcbb"
//! Output: 3 
//! Explanation: The answer is "abc", with the length of 3. 
//! ```
//! 
//! ## Example 2:
//! ```text
//! Input: "bbbbb"
//! Output: 1
//! Explanation: The answer is "b", with the length of 1.
//! ```
//!
//! ## Example 3:
//! ```text
//! Input: "pwwkew"
//! Output: 3
//! Explanation: The answer is "wke", with the length of 3.
//! Note that the answ  er must be a substring, "pwke" is a subsequence and not a substring.
//! ```
//!

pub type Input = String;
pub type Answer = i32;

pub trait Solution {
    fn length_of_longest_substring(&self, s: String) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;
impl Solution for Solution0 {

    fn length_of_longest_substring(&self, s: String) -> i32 {
        
        use std::collections::HashMap;
        use std::cmp::max;

        let mut map = HashMap::new();
        let mut max_length = 0;
        let mut current_length = 0;

        let characters = s.as_bytes();
        
        let mut i = 0;
        while i != characters.len() {

            let ch = characters[i].clone();

            //dbg!(ch as char);
            if let Some(index) = map.insert(ch, i) {
                current_length = 0;
                i = index + 1;
                map.clear();
            } else {
                current_length += 1;
                max_length = max(current_length, max_length);

                i += 1;
            }

            //dbg!(current_length);
        }

        max_length
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 1: Sliding Window Optimized
pub struct Solution1;
impl Solution for Solution1 {

    fn length_of_longest_substring(&self, s: String) -> i32 {
        
        use std::collections::HashMap;
        use std::cmp::max;

        let mut map: HashMap<u8, usize> = HashMap::new();
        let mut max_length = 0;

        let mut j = 0;
        let mut i = 0;

        for ch in s.as_bytes() {

            if let Some(index) = map.get(ch) {
                i = max(index.clone(), i);
            }

            let current_length = j - i + 1;
            max_length = max(current_length, max_length);

            j += 1;
            map.insert(ch.clone(), j);
        }
        
        max_length as i32
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Simplified version of Approach 1.
pub struct Solution2;
impl Solution for Solution2 {

    fn length_of_longest_substring(&self, s: String) -> i32 {
        
        use std::cmp::max;

        let mut map: [usize; 128] = [0; 128];
        let mut max_length = 0;

        let mut j = 0;
        let mut i = 0;

        for ch in s.as_bytes() {

            let ch = *ch as usize;

            i = max(map[ch], i);
            let current_length = j - i + 1;
            max_length = max(current_length, max_length);

            j += 1;
            map[ch] = j;
        }
        
        max_length as i32
    }
}
// -----------------------------------------------------------------------------
