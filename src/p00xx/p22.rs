//!
//! Generate Parentheses
//!
//! https://leetcode.com/problems/generate-parentheses/
//!
//! Given n pairs of parentheses, write a function to generate all combinations of well-formed parentheses.
//!
//! For example, given n = 3, a solution set is:
//! ```text
//! [
//!  "((()))",
//!  "(()())",
//!  "(())()",
//!  "()(())",
//!  "()()()"
//! ]
//! ```
//!


pub type Input  = i32;
pub type Output = Vec<String>;

pub trait Solution {
   fn generate_parenthesis(&self, n: i32) -> Vec<String>; 
}

// -----------------------------------------------------------------------------
/// Approach 0: Backtracking.
pub struct Solution0;
impl Solution for Solution0 {

    fn generate_parenthesis(&self, n: i32) -> Vec<String> {

        let mut result = Vec::new();
        gen_parenthesis(String::from("("), 1, n as usize, &mut result);
        result
    }
}

fn gen_parenthesis(pattern: String, i: usize, n: usize, result: &mut Vec<String>) {
    
    if pattern.len() == 2 * n {
        result.push(pattern.clone());
        return
    } else if i == n {
        gen_parenthesis(pattern + ")", i, n, result);
    } else {
        gen_parenthesis(pattern.clone() + "(", i + 1, n, result);

        if i > pattern.len() - i {
            gen_parenthesis(pattern + ")", i, n, result);
        }
    }
}
// -----------------------------------------------------------------------------

