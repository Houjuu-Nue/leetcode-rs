//!
//! Next Permutation
//!
//! https://leetcode.com/problems/next-permutation/
//!
//! Implement **next permutation**, which rearranges numbers into the lexicographically next greater permutation of numbers.
//!
//! If such arrangement is not possible, it must rearrange it as the lowest possible order (ie, sorted in ascending order).
//!
//! The replacement must be [in-place](http://en.wikipedia.org/wiki/In-place_algorithm) and use only constant extra memory.
//!
//! Here are some examples.
//! Inputs are in the left-hand column and its corresponding outputs are in the right-hand column.
//!
//! `1,2,3` → `1,3,2`
//!
//! `3,2,1` → `1,2,3`
//!
//! `1,1,5` → `1,5,1`
//!


pub type Input  = Vec<i32>;
pub type Output = ();

pub trait Solution {
    fn next_permutation(&self, nums: &mut Vec<i32>);
}

// -----------------------------------------------------------------------------
/// Approach 0: Search from end to start.
pub struct Solution0;
impl Solution for Solution0 {

    fn next_permutation(&self, nums: &mut Vec<i32>) {

         for i in (0..(nums.len() - 1)).rev() {
            for j in ((i + 1)..nums.len()).rev() {

                if nums[j] > nums[i] {
                    nums.swap(i, j);
                    nums[(i + 1)..].sort_unstable();
                    return
                }
            }
        }
        
        nums.sort_unstable();
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Single Pass Approach.
pub struct Solution1;
impl Solution for Solution1 {

    fn next_permutation(&self, nums: &mut Vec<i32>) {

        let len = nums.len();
        if len < 2 { return }

        let mut i = (len - 2) as i32;
        while i >= 0 {
            if nums[i as usize] < nums[(i + 1) as usize] { break }
            i -= 1;
        }

        if i < 0 {
            nums.sort_unstable();
            return
        }

        let i = i as usize;
        let mut j = len - 1;
        while i < j {
            if nums[i] < nums[j] {
                nums.swap(i, j);
                nums[(i + 1)..len].sort_unstable();
                return
            }
            j -= 1;
        }

   }
}
// -----------------------------------------------------------------------------

