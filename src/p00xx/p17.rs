//!
//! Letter Combinations of a Phone Number
//!
//! https://leetcode.com/problems/letter-combinations-of-a-phone-number/
//!
//! Given a string containing digits from `2-9` inclusive, return all possible letter combinations that the number could represent.
//!
//! A mapping of digit to letters (just like on the telephone buttons) is given below.
//!
//! Note that 1 does not map to any letters.
//!
//! ![Telephone-keypad2](https://upload.wikimedia.org/wikipedia/commons/thumb/7/73/Telephone-keypad2.svg/200px-Telephone-keypad2.svg.png)
//!
//! **Example:**
//! ```text
//! Input: "23"
//! Output: ["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"].
//! ```
//!
//! **Note:**
//! Although the above answer is in lexicographical order, your answer could be in any order you want.
//!
//!


pub type Input  = String;
pub type Output = Vec<String>;

pub trait Solution {
    fn letter_combinations(&self, digits: String) -> Vec<String>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force
pub struct Solution0;
impl Solution for Solution0 {

    fn letter_combinations(&self, digits: String) -> Vec<String> {

        let maps = generate_maps();
        if digits.is_empty() { return Vec::new() }
        let mut result = vec![String::new()];

        for digit in digits.chars() {
            let mut temp = Vec::new();

            let candidates = &maps[&digit];
            
            for prefix in result.iter() {
                for posfix in candidates.iter() {
                    temp.push(format!("{}{}", prefix, posfix));
                }
            }
            result = temp;
        }

        result
    }
}

use std::collections::HashMap;
fn generate_maps() -> HashMap<char, Vec<char>> {

    let mut maps = HashMap::new();
    
    maps.insert('2', vec!['a', 'b', 'c']);
    maps.insert('3', vec!['d', 'e', 'f']);
    maps.insert('4', vec!['g', 'h', 'i']);
    maps.insert('5', vec!['j', 'k', 'l']);
    maps.insert('6', vec!['m', 'n', 'o']);
    maps.insert('7', vec!['p', 'q', 'r', 's']);
    maps.insert('8', vec!['t', 'u', 'v']);
    maps.insert('9', vec!['w', 'x', 'y', 'z']);

    maps
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: DFS
pub struct Solution1;
impl Solution for Solution1 {

    fn letter_combinations(&self, digits: String) -> Vec<String> {
        
        if digits.is_empty() { return Vec::new() }
        let maps = generate_maps();
        let digits: Vec<char> = digits.chars().collect();
        
        let mut s = vec![0; digits.len()];
        let mut result = Vec::new();

        dfs(&maps, &mut s, &digits, 0, digits.len(), &mut result);
        
        result
    }
}

fn dfs(maps: &HashMap<char, Vec<char>>, s: &mut Vec<u8>, digits: &Vec<char>, depth: usize, max_depth: usize, result: &mut Vec<String>) {

    if depth == max_depth {
        let answer = unsafe {
            String::from_utf8_unchecked(s.clone())
        };
        result.push(answer);
    } else {
        let candidates = &maps[&digits[depth]];
        for digit in candidates.iter().cloned() {
            s[depth] = digit as u8;
            dfs(maps, s, digits, depth + 1, max_depth, result);
        }
    }
}
// -----------------------------------------------------------------------------

