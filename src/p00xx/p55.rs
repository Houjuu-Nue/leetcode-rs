//!
//! Jump Game
//!
//! https://leetcode.com/problems/jump-game/
//!
//! Given an array of non-negative integers, you are initially positioned at the first index of the array.
//!
//! Each element in the array represents your maximum jump length at that position.
//!
//! Determine if you are able to reach the last index.
//!
//! **Example 1:**
//! ```text
//! Input: [2,3,1,1,4]
//! Output: true
//! Explanation: Jump 1 step from index 0 to 1, then 3 steps to the last index.
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: [3,2,1,0,4]
//! Output: false
//! Explanation: You will always arrive at index 3 no matter what. Its maximum
//!              jump length is 0, which makes it impossible to reach the last index.
//! ```
//!



pub type Input  = Vec<i32>;
pub type Output = bool;

pub trait Solution {
    fn can_jump(&self, nums: Vec<i32>) -> bool;
}

// -----------------------------------------------------------------------------
/// Approach 0: Greedy Strategy.
pub struct Solution0;
impl Solution for Solution0 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {

        let mut last_pos = nums.len() - 1;

        for i in (0..nums.len()).rev() {
            if i + nums[i] as usize >= last_pos {
                last_pos = i;
            }
        }

        last_pos == 0
    }
}
// -----------------------------------------------------------------------------

// -----------------------------------------------------------------------------
/// Approach 1: Set visited.
pub struct Solution1;
impl Solution for Solution1 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {

        let mut can_visited = vec![false; nums.len()];
        can_visited[0] = true;
        
        for i in 0..nums.len() {
            if can_visited[i] == false { return false }
            let max_step = nums[i] as usize;

            if max_step + i >= nums.len() {
                return true
            } else {
                for j in (i + 1)..=(max_step + i) {
                    can_visited[j] = true;
                }
            }
        }

        true
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Backtracking.
pub struct Solution2;
impl Solution for Solution2 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {
        backtracking_can_jum_from(&nums)
    }
}

fn backtracking_can_jum_from(nums: &[i32]) -> bool {

    if nums[0] as usize + 1 >= nums.len() { return true }
    
    let jump_boundary = nums.len().min(nums[0] as usize + 1);

    (1..jump_boundary).rev().any(|i| backtracking_can_jum_from(&nums[i..]))
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 3: Dynamic Programming Top-Down.
pub struct Solution3;

#[derive(Debug, Clone, PartialEq, Eq)]
enum LocStatus { Good, Bad, Unknown }

impl Solution for Solution3 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {
        let mut status = vec![LocStatus::Unknown; nums.len()];
        (*status.last_mut().unwrap()) = LocStatus::Good;

        dyn_topdown_can_jum_from(&nums, &mut status)
    }
}

fn dyn_topdown_can_jum_from(nums: &[i32], status: &mut [LocStatus]) -> bool {

    match status[0] {
        | LocStatus::Good => true,
        | LocStatus::Bad  => false,
        | LocStatus::Unknown => {

            let jump_boundary = nums.len().min(nums[0] as usize + 1);

            let ans = (1..jump_boundary).any(|i| dyn_topdown_can_jum_from(&nums[i..], &mut status[i..]));

            if ans == false { status[0] = LocStatus::Bad; }
            
            ans
        },
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 4: Dynamic Programming Bottom-Up.
pub struct Solution4;
impl Solution for Solution4 {

    fn can_jump(&self, nums: Vec<i32>) -> bool {

        let mut status = vec![LocStatus::Unknown; nums.len()];
        (*status.last_mut().unwrap()) = LocStatus::Good;

        for i in (0..(nums.len() - 1)).rev() {
            let jump_boundary = (i + nums[i] as usize + 1).min(nums.len());

            for j in (i + 1)..jump_boundary {
                if status[j] == LocStatus::Good {
                    status[i] = LocStatus::Good;
                    break
                }
            }
        }

        status[0] == LocStatus::Good
    }
}
// -----------------------------------------------------------------------------

