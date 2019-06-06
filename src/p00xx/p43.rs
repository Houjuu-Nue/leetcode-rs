//!
//! Multiply Strings
//!
//! https://leetcode.com/problems/multiply-strings/
//!
//! Given two non-negative integers `num1` and `num2` represented as strings, return the product of `num1` and `num2`, also represented as a string.
//!
//! **Example 1:**
//! ```text
//! Input: num1 = "2", num2 = "3"
//! Output: "6"
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: num1 = "123", num2 = "456"
//! Output: "56088"
//! ```
//!
//! **Note:**
//! 
//! 1. The length of both `num1` and `num2` is < 110.
//!
//! 2. Both num1 and `num2` contain only digits `0-9`.
//!
//! 3. Both `num1` and `num2` do not contain any leading zero, except the number 0 itself.
//!
//! 4. You **must not use any built-in BigInteger library** or **convert the inputs to integer directly**.
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub num1: String,
    pub num2: String,
}
pub type Output = String;

pub trait Solution {
    fn multiply(&self, num1: String, num2: String) -> String;
}

// -----------------------------------------------------------------------------
/// Approach 0: Multiply Simulation(Burte Force).
pub struct Solution0;
impl Solution for Solution0 {

    fn multiply(&self, num1: String, num2: String) -> String {

        let mut num1: Vec<u8> = num1.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        let mut num2: Vec<u8> = num2.chars().map(|ch| ch.to_digit(10).unwrap() as u8).collect();
        if num1.len() < num2.len() { std::mem::swap(&mut num1, &mut num2); }
        if num2.len() == 1 && num2[0] == 0 { return String::from("0") }

        let mut ans: Vec<u8> = Vec::new();
        for (i, n2) in num2.into_iter().rev().enumerate() {
            
            let sum = num_multiply_digit(&num1, n2);
            plus_to(sum, i, &mut ans);
            // dbg!(&ans);
        }

        ans.into_iter().rev()
            .map(|digit| (digit + '0' as u8) as char)
            .collect()
    }
}

fn num_multiply_digit(num: &Vec<u8>, digit: u8) -> Vec<u8> {

    let mut carry = 0;
    let mut sum = Vec::new();

    for n in num.iter().rev().cloned() {
        let res = n * digit + carry;
        
        carry = res / 10;
        sum.push(res % 10);
    }
    if carry > 0 { sum.push(carry); }

    sum
}

fn plus_to(num: Vec<u8>, forward: usize, to: &mut Vec<u8>) {

    let mut carry = 0;
    let mut i = 0;
    
    for n in num.iter() {

        let pos = i + forward;
        if pos < to.len() {
            let res = to[pos] + n + carry;
            to[pos] = res % 10;
            carry = res / 10;
        } else {
            let res = n + carry;
            to.push(res % 10);
            carry = res / 10;
        }

        i += 1;
    }
    
    if carry > 0 && i + forward > num.len() {
        to.push(carry);
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Multiply Simulation(Simpilify).
pub struct Solution1;
impl Solution for Solution1 {

    fn multiply(&self, num1: String, num2: String) -> String {

        let num1: Vec<u32> = num1.chars().map(|ch| ch.to_digit(10).unwrap()).collect();
        let num2: Vec<u32> = num2.chars().map(|ch| ch.to_digit(10).unwrap()).collect();

        let mut ans: Vec<u32> = vec![0; num1.len() + num2.len()];

        for (i, n1) in num1.into_iter().rev().enumerate() {
            for (j, n2) in num2.iter().rev().cloned().enumerate() {
                ans[i + j] += n1 * n2;
            }
        }

        // handle carry
        let mut carry = 0;
        for i in 0..ans.len() {
            let sum = ans[i] + carry;
            carry  = sum / 10;
            ans[i] = sum % 10;
        }

        // clear front zero
        let first_non_zero_pos = ans.iter().rev().position(|&n| n != 0).unwrap_or(ans.len() - 1);
        ans.truncate(ans.len() - first_non_zero_pos);

        // convert to String
        ans.into_iter().rev()
            .map(|digit| (digit as u8 + '0' as u8) as char)
            .collect()
    }
}
// -----------------------------------------------------------------------------

