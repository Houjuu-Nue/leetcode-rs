//!
//! Spiral Matrix
//!
//! https://leetcode.com/problems/spiral-matrix/
//!
//! Given a matrix of m x n elements (m rows, n columns), return all elements of the matrix in spiral order.
//!
//! **Example 1:**
//! ```text
//! Input:
//! [
//!  [ 1, 2, 3 ],
//!  [ 4, 5, 6 ],
//!  [ 7, 8, 9 ]
//! ]
//! Output: [1,2,3,6,9,8,7,4,5]
//! ```
//!
//! **Example 2:**
//! ```text
//! Input:
//! [
//!   [1, 2, 3, 4],
//!   [5, 6, 7, 8],
//!   [9,10,11,12]
//! ]
//! Output: [1,2,3,4,8,12,11,10,9,5,6,7]
//! ```
//!



pub type Input  = Vec<Vec<i32>>;
pub type Output = Vec<i32>;

pub trait Solution {
    fn spiral_order(&self, matrix: Vec<Vec<i32>>) -> Vec<i32>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Layer-by-Layer.
pub struct Solution0;
impl Solution for Solution0 {

    fn spiral_order(&self, matrix: Vec<Vec<i32>>) -> Vec<i32> {

        if matrix.is_empty() { return Vec::new() }
        
        let mut left = 0;
        let mut right = matrix[0].len();
        let mut up = 0;
        let mut bottom = matrix.len();

        let mut ans = Vec::new();

        loop {
            if left >= right { break }
            for j in left..right {
                ans.push(matrix[up][j]);
            }
            up += 1;

            if up >= bottom { break }
            for i in up..bottom {
                ans.push(matrix[i][right - 1]);
            }
            right -= 1;

            if left >= right { break }
            for j in (left..right).rev() {
                ans.push(matrix[bottom - 1][j]);
            }
            bottom -= 1;

            if up >= bottom { break }
            for i in (up..bottom).rev() {
                ans.push(matrix[i][left]);
            }
            left += 1;
        }

        ans
    }
}
// -----------------------------------------------------------------------------

