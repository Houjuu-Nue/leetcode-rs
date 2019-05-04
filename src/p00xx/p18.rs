//!
//! 4Sum
//!
//! https://leetcode.com/problems/4sum/
//!
//! Given an array nums of n integers and an integer target, are there elements a, b, c, and d in nums such that a + b + c + d = target? Find all unique quadruplets in the array which gives the sum of target.
//!
//! **Note:**
//! The solution set must not contain duplicate quadruplets.
//!
//! **Example:**
//! ```text
//! Given array nums = [1, 0, -1, 0, -2, 2], and target = 0.
//!
//! A solution set is:
//! [
//!  [-1,  0, 0, 1],
//!  [-2, -1, 1, 2],
//!  [-2,  0, 0, 2].
//! ]
//! ```
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub nums: Vec<i32>,
    pub target: i32,
}
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn four_sum(&self, nums: Vec<i32>, target: i32) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Four Pointers(based on 3Sum Problem)
pub struct Solution0;
impl Solution for Solution0 {

    fn four_sum(&self, mut nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {

        if nums.len() < 4 { return Vec::new() }

        nums.sort();
        let mut results = Vec::new();

        for first in 0..(nums.len() - 3) {

            if first > 0 && nums[first] == nums[first - 1] {
                continue
            }

            for second in (first + 1)..(nums.len() - 2) {
                
                if second > first + 1 && nums[second] == nums[second - 1] {
                    continue
                }

                let mut third = second + 1;
                let mut forth = nums.len() - 1;

                while third < forth {
                    let sum = nums[first] + nums[second] + nums[third] + nums[forth];
                    // println!("{:?} {:?} {:?} {:?}", nums[first], nums[second], nums[third], nums[forth]);

                    if sum == target {

                        results.push(vec![nums[first], nums[second], nums[third], nums[forth]]);

                        while third < forth && nums[third] == nums[third + 1] {
                            third += 1;
                        }
                        while third < forth && nums[forth] == nums[forth - 1] {
                            forth -= 1;
                        }

                        third += 1;
                        forth -= 1;

                    } else if sum < target {
                        third += 1;
                    } else {
                        forth -= 1;
                    }

                }

            }
        }

        results

    }
}
// -----------------------------------------------------------------------------

