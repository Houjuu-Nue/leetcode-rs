//!
//! Container With Most Water
//!
//! https://leetcode.com/problems/regular-expression-matching/
//!
//! Given n non-negative integers a1, a2, ..., an , where each represents a point at coordinate (i, ai).
//! n vertical lines are drawn such that the two endpoints of line i is at (i, ai) and (i, 0).
//! Find two lines, which together with x-axis forms a container, such that the container contains the most water.
//! 
//! Note: You may not slant the container and n is at least 2.
//! 
//! ![](https://s3-lc-upload.s3.amazonaws.com/uploads/2018/07/17/question_11.jpg)
//!
//! The above vertical lines are represented by array [1,8,6,2,5,4,8,3,7].
//! In this case, the max area of water (blue section) the container can contain is 49. 
//!
//! ## Example:
//! ```text
//! Input: [1,8,6,2,5,4,8,3,7]
//! Output: 49
//! ```
//! 


pub type Input  = Vec<i32>;
pub type Output = i32;

pub trait Solution {
    fn max_area(&self, height: Vec<i32>) -> i32;
}

// -----------------------------------------------------------------------------
// Approach 0: Brute force
pub struct Solution0;
impl Solution for Solution0 {

    fn max_area(&self, height: Vec<i32>) -> i32 {

        let mut max_area = 0;

        for i in 0..height.len() {
            for j in (i + 1)..height.len() {

                let cur_area = area(&height, i, j);
                if cur_area > max_area {
                    max_area = cur_area;
                }
            }
        }

        max_area
    }
}

#[inline]
fn area(heights: &[i32], i: usize, j: usize) -> i32 {
    use std::cmp::min;
    min(heights[i], heights[j]) * ((j - i) as i32)
}
// -----------------------------------------------------------------------------
