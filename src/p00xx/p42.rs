//!
//! Trapping Rain Water
//!
//! https://leetcode.com/problems/trapping-rain-water/
//!
//! Given n non-negative integers representing an elevation map where the width of each bar is 1, compute how much water it is able to trap after raining.
//!
//! ![](https://assets.leetcode.com/uploads/2018/10/22/rainwatertrap.png)
//!
//! The above elevation map is represented by array [0,1,0,2,1,0,1,3,2,1,2,1].
//!
//! In this case, 6 units of rain water (blue section) are being trapped.
//!
//! Thanks Marcos for contributing this image!.
//!
//! **Example:**
//! ```text
//! Input: [0,1,0,2,1,0,1,3,2,1,2,1]
//! Output: 6
//! ```
//!


pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn trap(&self, height: Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
/// Approach 0: Search one by one.
pub struct Solution0;
impl Solution for Solution0 {

    fn trap(&self, height: Vec<i32>) -> i32 {

        let mut water_heights = Vec::new();
        let mut i = 0;

        // search until reaching the highest column
        while i < height.len() {
            let i_height = height[i];
            let mut next_higher_pos = i + 1;

            while next_higher_pos < height.len() && i_height >= height[next_higher_pos] {
                next_higher_pos += 1;
            }

            if next_higher_pos == height.len() {
                // the i-th column is the highest column
                water_heights.push(i_height);
                i += 1;
                break
            } else {
                // store the height of each column(with rain)
                for _ in i..next_higher_pos {
                    water_heights.push(i_height);
                }

                i = next_higher_pos;
            }
        }

        // continue to search the highest column
        while i < height.len() {
            let mut next_higher_pos = i;
            let mut max_height = height[i];

            for j in (i + 1)..height.len() {
                if height[j] > max_height {
                    max_height = height[j];
                    next_higher_pos = j;
                }
            }

            for _ in i..=next_higher_pos {
                water_heights.push(max_height);
            }

            i = next_higher_pos + 1;
        }


        let height_without_water: i32 = height.iter().sum();
        let height_with_water   : i32 = water_heights.iter().sum();

        height_with_water - height_without_water
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Brute Force.
pub struct Solution1;
impl Solution for Solution1 {

    fn trap(&self, height: Vec<i32>) -> i32 {

        let mut sum_water = 0;
        for i in 1..(height.len() - 1) {
            
            // Search the left part for max bar size.
            let max_left  = height[0..=i].iter().max().unwrap();
            // Search the right part for max bar size.
            let max_right = height[i..].iter().max().unwrap();

            let water_height = max_left.min(max_right) - height[i];
            sum_water += water_height;
        }

        sum_water
    }
}
// -----------------------------------------------------------------------------

