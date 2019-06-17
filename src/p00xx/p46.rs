//!
//! Permutations
//!
//! https://leetcode.com/problems/permutations/
//!
//! Given a collection of **distinct** integers, return all possible permutations.
//!
//! **Example:**
//! ```text
//! Input: [1,2,3]
//! Output:
//! [
//!   [1,2,3],
//!   [1,3,2],
//!   [2,1,3],
//!   [2,3,1],
//!   [3,1,2],
//!   [3,2,1]
//! ]
//! ```
//!



pub type Input  = Vec<i32>;
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn permute(&self, nums: Vec<i32>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0
pub struct Solution0;

use std::collections::HashMap;
impl Solution for Solution0 {

    fn permute(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut maps: HashMap<i32, bool> = nums.iter().cloned()
            .map(|num| (num, false)).collect();

        let mut result = Vec::new();
        let mut nums_ = Vec::new();
        recursive_permute(&mut nums_, &nums, &mut maps, &mut result);

        result
    }
}

fn recursive_permute(nums: &mut Vec<i32>, candidate: &Vec<i32>, maps: &mut HashMap<i32, bool>, result: &mut Vec<Vec<i32>>) {

    if nums.len() == maps.len() {
        result.push(nums.clone());
        return
    }

    for num in candidate {
        if maps[num] == false {
            *(maps.get_mut(&num).unwrap()) = true;

            nums.push(num.clone());
            recursive_permute(nums, candidate, maps, result);
            nums.pop();

            *(maps.get_mut(&num).unwrap()) = false;
        }
    }
}
// -----------------------------------------------------------------------------

