//!
//! Integer to Roman
//!
//! https://leetcode.com/problems/integer-to-roman/
//! 
//! Roman numerals are represented by seven different symbols: I, V, X, L, C, D and M.
//! 
//! ```text
//! Symbol       Value
//! I             1
//! V             5
//! X             10
//! L             50
//! C             100
//! D             500
//! M             1000
//! ```
//! 
//! For example, two is written as II in Roman numeral, just two one's added together.
//! Twelve is written as, XII, which is simply X + II.
//! The number twenty seven is written as XXVII, which is XX + V + II.
//! 
//! Roman numerals are usually written largest to smallest from left to right.
//! However, the numeral for four is not IIII. Instead, the number four is written as IV.
//! Because the one is before the five we subtract it making four.
//! The same principle applies to the number nine, which is written as IX.
//! There are six instances where subtraction is used:
//! 
//! - I can be placed before V (5) and X (10) to make 4 and 9. 
//! - X can be placed before L (50) and C (100) to make 40 and 90. 
//! - C can be placed before D (500) and M (1000) to make 400 and 900.
//! 
//! Given an integer, convert it to a roman numeral.
//! Input is guaranteed to be within the range from 1 to 3999.
//! 
//! ## Example 1:
//! ```text
//! Input: 3
//! Output: "III"
//! ```
//! 
//! ## Example 2:
//! ```text
//! Input: 4
//! Output: "IV"
//! ```
//!
//! ## Example 3:
//! ```text
//! Input: 9
//! Output: "IX"
//! ```
//! 
//! ## Example 4:
//! ```text
//! Input: 58
//! Output: "LVIII"
//! Explanation: L = 50, V = 5, III = 3.
//! ```
//! 
//! ## Example 5:
//! ```text
//! Input: 1994
//! Output: "MCMXCIV"
//! Explanation: M = 1000, CM = 900, XC = 90 and IV = 4.
//! ```
//!


pub type Input  = i32;
pub type Output = String;

pub trait Solution {
    fn int_to_roman(&self, num: i32) -> String;
}

// -----------------------------------------------------------------------------
/// Approach 0: enumerate all situation
pub struct Solution0;
impl Solution for Solution0 {

    fn int_to_roman(&self, num: i32) -> String {

        let mut num = num;
        let mut numbers = Vec::new();

        let digit1 = num % 10;
        match digit1 {
            | 1 => numbers.push(String::from("I")),
            | 2 => numbers.push(String::from("II")),
            | 3 => numbers.push(String::from("III")),
            | 4 => numbers.push(String::from("IV")),
            | 5 => numbers.push(String::from("V")),
            | 6 => numbers.push(String::from("VI")),
            | 7 => numbers.push(String::from("VII")),
            | 8 => numbers.push(String::from("VIII")),
            | 9 => numbers.push(String::from("IX")),
            | _ => {},
        }
        num /= 10;

        if num > 0 {
            let digit2 = num % 10;
            match digit2 {
                | 1 => numbers.push(String::from("X")),
                | 2 => numbers.push(String::from("XX")),
                | 3 => numbers.push(String::from("XXX")),
                | 4 => numbers.push(String::from("XL")),
                | 5 => numbers.push(String::from("L")),
                | 6 => numbers.push(String::from("LX")),
                | 7 => numbers.push(String::from("LXX")),
                | 8 => numbers.push(String::from("LXXX")),
                | 9 => numbers.push(String::from("XC")),
                | _ => {},
            }
            num /= 10;
        }

        if num > 0 {
            let digit3 = num % 10;
            match digit3 {
                | 1 => numbers.push(String::from("C")),
                | 2 => numbers.push(String::from("CC")),
                | 3 => numbers.push(String::from("CCC")),
                | 4 => numbers.push(String::from("CD")),
                | 5 => numbers.push(String::from("D")),
                | 6 => numbers.push(String::from("DC")),
                | 7 => numbers.push(String::from("DCC")),
                | 8 => numbers.push(String::from("DCCC")),
                | 9 => numbers.push(String::from("CM")),
                | _ => {},
            }
            num /= 10;
        }

        if num > 0 {
            let digit4 = num % 10;
            match digit4 {
                | 1 => numbers.push(String::from("M")),
                | 2 => numbers.push(String::from("MM")),
                | 3 => numbers.push(String::from("MMM")),
                | _ => {},
            }
        }

        numbers.into_iter()
            .rev()
            .fold(String::new(), |sum, number| sum + &number)
    }
}
// -----------------------------------------------------------------------------
