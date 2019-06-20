//!
//! Group Anagrams
//!
//! https://leetcode.com/problems/group-anagrams/
//!
//! Given an array of strings, group anagrams together.
//!
//! **Example:**
//! ```text
//! Input: ["eat", "tea", "tan", "ate", "nat", "bat"],
//! Output:
//! [
//!   ["ate","eat","tea"],
//!   ["nat","tan"],
//!   ["bat"]
//! ]
//! ```
//!
//! **Note:**
//!
//! - All inputs will be in lowercase.
//!
//! - The order of your output does not matter.
//!



pub type Input  = Vec<String>;
pub type Output = Vec<Vec<String>>;

pub trait Solution {
    fn group_anagrams(&self, strs: Vec<String>) -> Vec<Vec<String>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn group_anagrams(&self, strs: Vec<String>) -> Vec<Vec<String>> {

        use std::collections::HashMap;

        let mut result: HashMap<Vec<u8>, Vec<String>> = HashMap::new();
        for s in strs {
            let mut str_sorted = s.clone().into_bytes();
            str_sorted.sort_unstable();

            result.entry(str_sorted).or_insert(Vec::new())
                .push(s);
        }

        result.into_iter()
            .map(|(_, strs)| strs).collect()
    }
}
// -----------------------------------------------------------------------------

