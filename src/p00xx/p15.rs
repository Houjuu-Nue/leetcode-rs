//!
//! 3Sum
//!
//! https://leetcode.com/problems/3sum/
//! 
//! Given an array `nums` of n integers, are there elements a, b, c in `nums` such that a + b + c = 0?
//!
//! Find all unique triplets in the array which gives the sum of zero.
//!
//! **Note**:
//!
//! The solution set must not contain duplicate triplets.
//!
//! **Example:**
//! ```text
//! Given array nums = [-1, 0, 1, 2, -1, -4]
//!
//! A solution set is:
//! [
//!   [-1, 0, 1],
//!   [-1, -1, 2]
//! ]
//! ```
//!

pub type Input  = Vec<i32>;
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn three_sum(&self, nums: Vec<i32>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Brute Force.
pub struct Solution0;
impl Solution for Solution0 {

    fn three_sum(&self, nums: Vec<i32>) -> Vec<Vec<i32>> {

        use std::collections::HashSet;
        
        let mut results = Vec::new();
        let mut test_set = HashSet::new();

        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {

                for k in (j + 1)..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {

                        if test_set.contains(&(nums[i], nums[j], nums[k])) == false {
                            results.push(vec![nums[i], nums[j], nums[k]]);

                            test_set.insert((nums[i], nums[j], nums[k]));
                            test_set.insert((nums[i], nums[k], nums[j]));
                            test_set.insert((nums[j], nums[i], nums[k]));
                            test_set.insert((nums[j], nums[k], nums[i]));
                            test_set.insert((nums[k], nums[i], nums[j]));
                            test_set.insert((nums[k], nums[j], nums[i]));
                        }

                    }
                }
            }
        }

        results
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: triplet pointers.
pub struct Solution1;
impl Solution for Solution1 {

    fn three_sum(&self, mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        use std::collections::HashSet;

        if nums.len() < 3 { return Vec::new() }

        nums.sort();
        let mut test_set = HashSet::new();
        let mut results = Vec::new();

        for start in 0..(nums.len() - 2) {

            let mut middle = start + 1;
            let mut end = nums.len() - 1;

            while middle < end {
                let sum = nums[start] + nums[middle] + nums[end];

                if sum == 0 {
                    
                    let nums_test = (nums[start], nums[middle], nums[end]);
                    if test_set.contains(&nums_test) == false {
                        // println!("{} {} {}", start, middle, end);
                        results.push(vec![nums[start], nums[middle], nums[end]]);
                        test_set.insert(nums_test);
                    }

                    middle += 1;
                    end -= 1;

                } else if sum < 0 {
                    middle += 1;
                } else { // sum > 0
                    end -= 1;
                }
            }
        }

        results
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Simplified version of Approach 1.
pub struct Solution2;
impl Solution for Solution2 {

    fn three_sum(&self, mut nums: Vec<i32>) -> Vec<Vec<i32>> {

        if nums.len() < 3 { return Vec::new() }

        nums.sort();
        let mut results = Vec::new();

        for start in 0..(nums.len() - 2) {

            if start > 0 && nums[start] == nums[start - 1] {
                continue
            }

            let mut middle = start + 1;
            let mut end = nums.len() - 1;

            while middle < end {
                let sum = nums[start] + nums[middle] + nums[end];

                if sum == 0 {

                    results.push(vec![nums[start], nums[middle], nums[end]]);

                    while middle < end && nums[middle] == nums[middle + 1] {
                        middle += 1;
                    }
                    while middle < end && nums[end] == nums[end - 1] {
                        end -= 1;
                    }

                    middle += 1;
                    end -= 1;

                } else if sum < 0 {
                    middle += 1;
                } else { // sum > 0
                    end -= 1;
                }
            }
        }

        results
    }
}
// -----------------------------------------------------------------------------
