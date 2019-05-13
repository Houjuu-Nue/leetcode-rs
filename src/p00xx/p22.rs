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
        result.push(pattern);
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

// -----------------------------------------------------------------------------
/// Approach 1: Brute Force.
pub struct Solution1;
impl Solution for Solution1 {

    fn generate_parenthesis(&self, n: i32) -> Vec<String> {

        let mut result = Vec::new();
        let mut pattern = String::new();
        gen_parenthesis_v1(&mut pattern, n as usize, &mut result);
        result
    }
}

fn gen_parenthesis_v1(pattern: &mut String, n: usize, result: &mut Vec<String>) {

    if pattern.len() == 2 * n {
        if is_valid(pattern) {
            result.push(pattern.clone());
        }
    } else {
        pattern.push('(');
        gen_parenthesis_v1(pattern, n, result);
        pattern.pop();

        pattern.push(')');
        gen_parenthesis_v1(pattern, n, result);
        pattern.pop();
    }
}

fn is_valid(s: &String) -> bool {

    let mut balance = 0;
    for ch in s.chars() {
        match ch {
            | '(' => balance += 1,
            | ')' => balance -= 1,
            | _   => unreachable!(),
        }

        if balance < 0 {
            return false
        }
    }

    balance == 0
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 2: Closure Number.
pub struct Solution2;
impl Solution for Solution2 {

    fn generate_parenthesis(&self, n: i32) -> Vec<String> {

        gen_parenthesis_v2(n as usize)
    }
}

fn gen_parenthesis_v2(n: usize) -> Vec<String> {

    if n == 0 {
        vec![String::new()]
    } else {
        let mut ans = Vec::new();
        for i in 0..n {
           for left in gen_parenthesis_v2(i) {
               for right in gen_parenthesis_v2(n - 1 - i) {
                   ans.push(format!("({}){}", &left, &right));
               }
           }
        }

        ans
    }
}
// -----------------------------------------------------------------------------

