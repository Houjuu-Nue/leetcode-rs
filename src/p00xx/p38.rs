//!
//! Count and Say
//!
//! https://leetcode.com/problems/count-and-say/
//!
//! The count-and-say sequence is the sequence of integers with the first five terms as following:
//!
//! ```text
//! 1.  1
//! 2.  11
//! 3.  21
//! 4.  1211
//! 5.  111221
//! ```
//!
//! `1` is read off as `"one 1"` or `11`.
//!
//! `11` is read off as `"two 1s"` or `21`.
//!
//! `21` is read off as `"one 2, then one 1"` or `1211`.
//!
//! Given an integer n where 1 ≤ *n* ≤ 30, generate the nth term of the count-and-say sequence.
//!
//! Note: Each term of the sequence of integers will be represented as a string.
//!
//! **Example 1:**
//! ```text
//! Input: 1
//! Output: "1"
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: 4
//! Output: "1211"
//! ```
//!


pub type Input  = i32;
pub type Output = String;

pub trait Solution {
    fn count_and_say(&self, n: i32) -> String;
}

// -----------------------------------------------------------------------------
/// Approach 0: Count one by one.
pub struct Solution0;
impl Solution for Solution0 {

    fn count_and_say(&self, n: i32) -> String {

        let mut digits = String::from("1");
        for _ in 1..(n as usize) {
            digits = count(digits);
        }

        digits
    }
}

fn count(digits: String) -> String {

    let mut iter = digits.chars();
    let mut result = String::new();

    let mut character = iter.next().unwrap();
    let mut count: u8 = 0b1;

    for ch in iter { 
        if ch == character {
            count += 1;
        } else {
            result.push(conver_count(count));
            result.push(character);

            character = ch;
            count = 1;
        }
    }
    result.push(conver_count(count));
    result.push(character);

    result
}
// -----------------------------------------------------------------------------

fn conver_count(count: u8) -> char {
    match count {
        | 1 => '1',
        | 2 => '2',
        | 3 => '3',
        _ => panic!()
    }
}

