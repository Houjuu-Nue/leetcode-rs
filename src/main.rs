//!
//! https://leetcode-cn.com/problems/two-sum/
//!
//! Given an array of integers, return indices of the two numbers such that they add up to a specific target.
//!
//! You may assume that each input would have exactly one solution, and you may not use the same element twice.
//!
//! Example:
//! ```
//! Given nums = [2, 7, 11, 15], target = 9,
//!
//! Because nums[0] + nums[1] = 2 + 7 = 9,
//! return [0, 1].
//! ```
//!

struct Input {
    nums: Vec<i32>,
    target: i32,
}

struct Solution;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        for i in 0..(nums.len() - 1) {
            
            for j in (i + 1)..nums.len() {
                let result = nums[i] + nums[j];

                if result == target {
                    return vec![i as i32, j as i32];
                }
            }
        }
        
        Vec::new()
    }
}

fn main() {

    let input = Input {
        nums: vec![0, 4, 3, 0],
        target: 0,
    };

    let result = Solution::two_sum(input.nums, input.target);

    if result == vec![0, 1] {
        println!("Ok");
    } else {
        println!("False");
    }
}
