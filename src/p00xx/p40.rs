//!
//! Combination Sum II
//!
//! https://leetcode.com/problems/combination-sum-ii/
//!
//! Given a collection of candidate numbers (`candidates`) and a target number (`target`), find all unique combinations in candidates where the candidate numbers sums to `target`.
//!
//! Each number in `candidates` may only be used **once** in the combination.
//!
//! **Note:**
//!
//! - All numbers (including `target`) will be positive integers.
//!
//! - The solution set must not contain duplicate combinations.
//!
//! **Example 1:**
//! ```text
//! Input: candidates = [10, 1, 2, 7, 6, 1, 5], target = 8,
//! A solution set is:
//! [
//!   [1, 7],
//!   [1, 2, 5],
//!   [2, 6],
//!   [1, 1, 6]
//! ]
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: candidates = [2, 5, 2, 1, 2], target = 5,
//! A solution set is:
//! [
//!   [1,2,2],
//!   [5]
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
    fn combination_sum2(&self, candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Dynamic Programming.
pub struct Solution0;
impl Solution for Solution0 {

    fn combination_sum2(&self, mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        
        let mut result = Vec::new();
        let mut groups = Vec::new();
        candidates.sort_unstable();
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

                // pick the i-th number
                groups.push(candidates[i]);
                fill_bag(candidates, i + 1, sum + candidates[i], target, groups, result);
                groups.pop();

                let mut next_pos = i + 1;
                while next_pos < candidates.len() && candidates[i] == candidates[next_pos] {
                    next_pos += 1;
                }

                // not to pick i-th number
                fill_bag(candidates, next_pos, sum, target, groups, result);
            }
       },
    }
}
// -----------------------------------------------------------------------------


