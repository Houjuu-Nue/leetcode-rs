//!
//! Combination Sum
//!
//! https://leetcode.com/problems/combination-sum/
//!
//! Given a **set** of candidate numbers (`candidates`) (**without duplicates**) and a target number (`target`), find all unique combinations in candidates where the candidate numbers sums to `target`.
//!
//! The same repeated number may be chosen from `candidates` unlimited number of times.
//!
//! **Note:**
//!
//! - All numbers (including `target`) will be positive integers.
//!
//! - The solution set must not contain duplicate combinations.
//!
//! **Example 1:**
//! ```text
//! Input: candidates = [2,3,6,7], target = 7,
//! A solution set is:
//! [
//!   [7],
//!   [2,2,3]
//! ]
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: candidates = [2,3,5], target = 8,
//! A solution set is:
//! [
//!   [2,2,2,2],
//!   [2,3,3],
//!   [3,5]
//! ]
//! ```
//!



#[derive(Debug, Clone)]
pub struct Input {
    pub candidates: Vec<i32>,
    pub target: i32,
}
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn combination_sum(&self, candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Dynamic Programming.
pub struct Solution0;
impl Solution for Solution0 {

    fn combination_sum(&self, candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        
        let mut result = Vec::new();
        let mut groups = Vec::new();
        fill_bag(&candidates, 0, 0, target, &mut groups, &mut result);

        result
    }
}

fn fill_bag(candidates: &Vec<i32>, i: usize, sum: i32, target: i32, groups: &mut Vec<i32>, result: &mut Vec<Vec<i32>>) {

    use std::cmp::Ordering;
    
    match sum.cmp(&target) {
        | Ordering::Equal => result.push(groups.clone()),
        | Ordering::Greater => {},
        | Ordering::Less => {
            
            if i < candidates.len() {

                // try pick the i-th number
                groups.push(candidates[i]);
                fill_bag(candidates, i, sum + candidates[i], target, groups, result);
                groups.pop();

                // try pick the next number
                if i + 1 < candidates.len() {
                    fill_bag(candidates, i + 1, sum, target, groups, result);
                }
            }
       },
    }
}
// -----------------------------------------------------------------------------

