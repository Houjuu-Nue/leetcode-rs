//!
//! Median of Two Sorted Arrays
//!
//! https://leetcode.com/problems/median-of-two-sorted-arrays/
//!
//! There are two sorted arrays nums1 and nums2 of size m and n respectively.
//!
//! Find the median of the two sorted arrays. The overall run time complexity should be O(log (m+n)).
//!
//! You may assume nums1 and nums2 cannot be both empty.
//!
//! ## Example 1:
//! ```
//! nums1 = [1, 3]
//! nums2 = [2]
//!  
//! The median is 2.0
//! ```
//!
//! ## Example 2:
//! ```
//! nums1 = [1, 2]
//! nums2 = [3, 4]
//! 
//! The median is (2 + 3)/2 = 2.5
//! ```
//!

#[derive(Debug, Clone)]
pub struct Input {
    pub nums1: Vec<i32>,
    pub nums2: Vec<i32>
}

pub type Answer = f64;

pub trait Solution {
    fn find_median_sorted_arrays(&self, nums1: Vec<i32>, nums2: Vec<i32>) -> f64;
}

// Approach 1: Brute Force
pub struct Solution1;
impl Solution for Solution1 {

    fn find_median_sorted_arrays(&self, nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {

        enum SampleNum { N1, N2 }
        
        let len1 = nums1.len();
        let len2 = nums2.len();

        let mut flag1 = (len1 + len2) % 2 != 0;
        let flag2 = flag1;
        let middle = (len1 + len2 - 1) / 2;

        let mut i  = 0;
        let mut p1 = 0;
        let mut p2 = 0;
        let mut sum: f64 = 0.0;

        'MARK: loop {

            // Decide sample number from nums1 or nums2.
            let path = match (p1, p2) {
                | (l1, _) if l1 == len1             => SampleNum::N2,
                | (_, l2) if l2 == len2             => SampleNum::N1,
                | (p1, p2) if nums1[p1] < nums2[p2] => SampleNum::N1,
                | _                                 => SampleNum::N2,
            };
            
            match path {
                | SampleNum::N1 => {
                    
                    if i >= middle {
                        sum += nums1[p1] as f64;

                        if flag1 {
                            break 'MARK
                        } else {
                            flag1 = true;
                        }
                    }

                    p1 += 1;
                },
                | SampleNum::N2 => {
                    
                    if i >= middle {
                        sum += nums2[p2] as f64;

                        if flag1 {
                            break 'MARK
                        } else {
                            flag1 = true;
                        }
                    }

                    p2 += 1;
                },
            }

            i += 1;
        }

        // calculate the median
        if flag2 {
            sum
        } else {
            sum / 2.0
        }
    }
}
