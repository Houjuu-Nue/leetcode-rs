//!
//! Substring with Concatenation of All Words
//!
//! https://leetcode.com/problems/substring-with-concatenation-of-all-words/
//!
//! You are given a string, s, and a list of words, **words**, that are all of the same length.
//!
//! Find all starting indices of substring(s) in s that is a concatenation of each word in **words** exactly once and without any intervening characters.
//!
//! **Example 1:**
//! ```text
//! Input:
//!   s = "barfoothefoobarman",
//!   words = ["foo","bar"]
//! Output: [0,9]
//! Explanation: Substrings starting at index 0 and 9 are "barfoor" and "foobar" respectively.
//! The output order does not matter, returning [9,0] is fine too.
//! ```
//!
//! **Example 2:**
//! ```text
//! Input:
//!   s = "wordgoodgoodgoodbestword",
//!   words = ["word","good","best","word"]
//! Output: []
//! ```
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub s: String,
    pub words: Vec<String>,
}
pub type Output = Vec<i32>;

pub trait Solution {
    fn find_substring(&self, s: String, words: Vec<String>) -> Vec<i32>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Recursive search.
pub struct Solution0;
impl Solution for Solution0 {

    fn find_substring(&self, s: String, words: Vec<String>) -> Vec<i32> {

        if words.is_empty() { return Vec::new() }
        let mut result = Vec::new();
        let mut candidates = vec![false; words.len()];

        let total_length = words.iter().map(|w| w.len()).sum();
        
        for i in 0..s.len() {
            if s.len() - i < total_length { break }
            find(&s[i..], &words, &mut candidates, i, 0, &mut result);
        }

        result
    }
}

fn find(substr: &str, words: &Vec<String>, candidates: &mut Vec<bool>, start: usize, used_count: usize, result: &mut Vec<i32>) -> bool {

    if used_count == words.len() {
        result.push(start as i32);
        return true
    }

    for i in 0..candidates.len() {

        // if words[i] is used, find next word.
        if candidates[i] == true { continue }
        
        let test_word = &words[i];

        if substr.starts_with(test_word) {

            candidates[i] = true;
            let is_found = find(&substr[test_word.len()..], words, candidates, start, used_count + 1, result);
            candidates[i] = false;

            if is_found { return true }
        }
    }

    false
}
// -----------------------------------------------------------------------------

