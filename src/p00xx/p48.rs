//!
//! Rotate Image
//!
//! https://leetcode.com/problems/rotate-image/
//!
//! You are given an n x n 2D matrix representing an image.
//!
//! Rotate the image by 90 degrees (clockwise).
//!
//! **Note:**
//!
//! You have to rotate the image ![in-place](https://en.wikipedia.org/wiki/In-place_algorithm), which means you have to modify the input 2D matrix directly.
//!
//! **DO NOT** allocate another 2D matrix and do the rotation.
//!
//! **Example 1:**
//! ```text
//! Given input matrix = 
//! [
//!   [1,2,3],
//!   [4,5,6],
//!   [7,8,9]
//! ],
//!
//! rotate the input matrix in-place such that it becomes:
//! [
//!  [7,4,1],
//!  [8,5,2],
//!  [9,6,3]
//! ]
//! ```
//!
//! **Example 2:**
//! ```text
//! Given input matrix =
//! [
//!   [ 5, 1, 9,11],
//!   [ 2, 4, 8,10],
//!   [13, 3, 6, 7],
//!   [15,14,12,16]
//! ], 
//!
//! rotate the input matrix in-place such that it becomes:
//! [
//!  [15,13, 2, 5],
//!  [14, 3, 4, 1],
//!  [12, 6, 8, 9],
//!  [16, 7,10,11]
//! ]
//! ```
//!


pub type Input  = Vec<Vec<i32>>;
pub type Output = ();

pub trait Solution {
    fn rotate(&self, matrix: &mut Vec<Vec<i32>>);
}

// -----------------------------------------------------------------------------
/// Approach 0: Filp and Mirror.
pub struct Solution0;
impl Solution for Solution0 {

    fn rotate(&self, matrix: &mut Vec<Vec<i32>>) {
        
        let dimension = matrix.len();

        for i in 0..dimension {
            for j in (i + 1)..dimension {
                
                let temp = matrix[i][j];
                matrix[i][j] = matrix[j][i];
                matrix[j][i] = temp;
            }
        }

        for row in matrix.iter_mut() {
            row.reverse();
        }
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Formal Rotation.
pub struct Solution1;
impl Solution for Solution1 {

    fn rotate(&self, matrix: &mut Vec<Vec<i32>>) {

        let mut bound1 = 0;
        let mut bound2 = matrix.len() - 1;
        
        while bound1 < bound2 {
            let mut i = bound1;
            let mut j = bound2;

            while i != bound2 {
                
                let temp = matrix[bound1][i];          // save top-left
                matrix[bound1][i] = matrix[j][bound1]; // top-left     = bottom-left
                matrix[j][bound1] = matrix[bound2][j]; // bottom-left  = bottom-right
                matrix[bound2][j] = matrix[i][bound2]; // bottom-right = top-right
                matrix[i][bound2] = temp;              // top-right    = top-left
                
                i += 1;
                j -= 1;
            }

            bound1 += 1;
            bound2 -= 1;
        }
    }
}
// -----------------------------------------------------------------------------

