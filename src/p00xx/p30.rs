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
use std::collections::HashMap;

pub struct Solution0;
impl Solution for Solution0 {

    fn find_substring(&self, s: String, words: Vec<String>) -> Vec<i32> {

        if words.is_empty() { return Vec::new() }
        let mut result = Vec::new();
        let mut candidates = HashMap::new();

        let word_length = words[0].len();
        let words_count = words.len();
        let total_length = word_length * words_count;
        
        for word in words.iter() {
            let word_count = candidates.entry(word.as_str()).or_insert(0);
            (*word_count) += 1;
        }

        for i in 0..s.len() {
            if s.len() - i < total_length { break }
            find(&s[i..], &mut candidates, words_count, word_length, i, 0, &mut result);
        }

        result
    }
}

fn find(
    substr: &str, candidates: &mut HashMap<&str, usize>, 
    words_count: usize, word_length: usize, start: usize, used_count: usize, 
    result: &mut Vec<i32>) -> bool {

    if substr.len() < word_length { return false }

    let test_substr: &str = &substr[..word_length];
    
    if let Some(word_count) = candidates.get_mut(test_substr) {
        if (*word_count) > 0 {

            if used_count + 1 == words_count {
                result.push(start as i32);
                return true
            }

            (*word_count) -= 1;
            let is_found = find(&substr[word_length..], candidates, words_count, word_length, start, used_count + 1, result);
            
            // avoid borrow checker complain.
            //(*word_count) += 1;
            if let Some(word_count) = candidates.get_mut(test_substr) {
                (*word_count) += 1;
            }

            is_found
        } else {
            return false
        }
    } else {
        false
    }
}
// -----------------------------------------------------------------------------

