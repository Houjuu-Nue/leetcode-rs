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
/// Approach 0: Hash with sorted numbers.
pub struct Solution0;
impl Solution for Solution0 {

    fn group_anagrams(&self, strs: Vec<String>) -> Vec<Vec<String>> {
        hash_for_group_anagrams(strs, hash_with_sorted_numbers)
    }
}

fn hash_for_group_anagrams<Key, HashFunc>(strs: Vec<String>, hash_key: HashFunc) -> Vec<Vec<String>>
    where
        Key: std::hash::Hash + Eq,
        HashFunc: Fn(&String) -> Key {

    use std::collections::HashMap;

    let mut result: HashMap<Key, Vec<String>> = HashMap::new();
    for s in strs {
        let key = hash_key(&s);
        result.entry(key).or_insert(Vec::new())
            .push(s);
    }

    result.into_iter()
        .map(|(_, strs)| strs).collect()
}

fn hash_with_sorted_numbers(key: &String) -> Vec<u8> {

    let mut str_sorted = key.clone().into_bytes();
    str_sorted.sort_unstable();
    str_sorted
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Hash with prime numbers multiplication.
pub struct Solution1;

// Each prime number represent a letter.
const PRIME_NUMBERS: [u64; 26] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29,
    31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    73, 79, 83, 89, 97, 101,
];

impl Solution for Solution1 {

    fn group_anagrams(&self, strs: Vec<String>) -> Vec<Vec<String>> {
        hash_for_group_anagrams(strs, hash_with_primes)
    }
}

fn hash_with_primes(key: &String) -> u64 {
    key.chars().map(|ch| {
        let index = (ch as u8 - 'a' as u8) as usize;
        PRIME_NUMBERS[index]
    }).product()
}
// -----------------------------------------------------------------------------

