//!
//! Permutations II
//!
//! https://leetcode.com/problems/permutations-ii/
//!
//! Given a collection of numbers that might contain duplicates, return all possible unique permutations.
//!
//! **Example:**
//! ```text
//! Input: [1,1,2]
//! Output:
//! [
//!   [1,1,2],
//!   [1,2,1],
//!   [2,1,1]
//! ]
//! ```
//!



pub type Input  = Vec<i32>;
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn permute_unique(&self, nums: Vec<i32>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: DFS.
pub struct Solution0;
impl Solution for Solution0 {

    fn permute_unique(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut used = vec![false; nums.len()];
        let mut result = Vec::new();
        let mut _nums = Vec::new();

        dfs(&mut _nums, &nums, &mut used, &mut result);

        result
    }
}

fn dfs(nums: &mut Vec<i32>, candidate: &[i32], used: &mut [bool], result: &mut Vec<Vec<i32>>) {

    if nums.len() == candidate.len() {
        result.push(nums.clone());
        return
    }

    use std::collections::HashSet;
    let mut set = HashSet::new();

    for (i, num) in candidate.iter().enumerate() {
        if used[i] == false && set.contains(&num) == false {
            used[i] = true;

            nums.push(num.clone());
            dfs(nums, candidate, used, result);
            nums.pop();

            used[i] = false;
            set.insert(num);
        }
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Swap Recursivily.
pub struct Solution1;
impl Solution for Solution1 {

    fn permute_unique(&self, mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        let mut result = Vec::new();
        swap_recursivily(&mut nums, 0, &mut result);

        result
    }
}

fn swap_recursivily(nums: &mut Vec<i32>, current: usize, result: &mut Vec<Vec<i32>>) {

    use std::collections::HashSet;
    let mut set = HashSet::new();

    if current == nums.len() {
        result.push(nums.clone());
    } else {
        for i in current..nums.len() {
            if set.contains(&nums[i]) == false {

                nums.swap(i, current);
                swap_recursivily(nums, current + 1, result);
                nums.swap(i, current);
                
                set.insert(nums[i]);
            }
        }
    }
}
// -----------------------------------------------------------------------------

