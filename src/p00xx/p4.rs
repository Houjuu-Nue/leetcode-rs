//!
//! Median of Two Sorted Arrays
//!
//! https://leetcode.com/problems/median-of-two-sorted-arrays/
//!
//! There are two sorted arrays nums1 and nums2 of size m and n respectively.
//!
//! Find the median of the two sorted arrays.
//!
//!The overall run time complexity should be O(log (m+n)).
//!
//! You may assume nums1 and nums2 cannot be both empty.
//!
//! **Example 1:**
//! ```text
//! nums1 = [1, 3]
//! nums2 = [2]
//!  
//! The median is 2.0
//! ```
//!
//! **Example 2:**
//! ```text
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

// -----------------------------------------------------------------------------
/// Approach 0: Merge and select the median.
pub struct Solution0;
impl Solution for Solution0 {

    fn find_median_sorted_arrays(&self, nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
        let nums = merge_nums(nums1, nums2);
        let len = nums.len();

        (nums[(len - 1) / 2] + nums[len / 2]) as f64 / 2.0
    }
}

fn merge_nums(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {

    let mut nums = Vec::with_capacity(nums1.len() + nums2.len());
    
    let mut l1 = 0;
    let mut l2 = 0;

    while l1 < nums1.len() && l2 < nums2.len() {
        if nums1[l1] <= nums2[l2] {
            nums.push(nums1[l1]);
            l1 += 1;
        } else {
            nums.push(nums2[l2]);
            l2 += 1;
        }
    }

    for i in l1..nums1.len() { nums.push(nums1[i]); }
    for i in l2..nums2.len() { nums.push(nums2[i]); }

    nums
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: Improve of Approach 0, Count number from min to median
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
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 2: Discard k / 2 elements each loop.
pub struct Solution2;
impl Solution for Solution2 {

    fn find_median_sorted_arrays(&self, nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        
        let length = nums1.len() + nums2.len();
        if length % 2 == 0 {
            (
                find_k_th(&nums1, 0, &nums2, 0, length / 2)
                +
                find_k_th(&nums1, 0, &nums2, 0, length / 2 + 1)
            ) / 2.0
        } else {
            find_k_th(&nums1, 0, &nums2, 0, length / 2 + 1)
        }

    }
}

fn find_k_th(nums1: &[i32], start1: usize, nums2: &[i32], start2: usize, k: usize) -> f64 {

    use std::cmp::{min, Ord, Ordering};

    let len1 = nums1.len() - start1;
    let len2 = nums2.len() - start2;

    if len1 > len2 {
        // make sure len1 < len2
        return find_k_th(nums2, start2, nums1, start1, k)
    } else if len1 == 0 { 
        return nums2[start2 + k - 1] as f64
    } else if k == 1 {
        return min(nums1[start1], nums2[start2]) as f64
    }

    // divide k into two parts
    let mid1 = min(k / 2, len1);
    let mid2 = k - mid1;

    match nums1[start1 + mid1 - 1].cmp(&nums2[start2 + mid2 - 1]) {
        | Ordering::Less => {
            // discard elements from nums1[start1] to nums1[start1 + mid1 - 1]
            find_k_th(nums1, start1 + mid1, nums2, start2, k - mid1)
        },
        | Ordering::Greater => {
            // discard elements from nums2[start2] to nums2[start2 + mid2 - 1]
            find_k_th(nums1, start1, nums2, start2 + mid2, k - mid2)
        },
        | Ordering::Equal => {
            // nums1[start1 + mid1 - 1] or nums2[start2 + mid2 - 1]
            // is just the median we are looking for.
            nums1[start1 + mid1 - 1] as f64
        }
    }
}
// -----------------------------------------------------------------------------
