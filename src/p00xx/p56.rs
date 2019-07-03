//!
//! Merge Intervals
//!
//! https://leetcode.com/problems/merge-intervals/
//!
//! Given a collection of intervals, merge all overlapping intervals.
//!
//! **Example 1:**
//! ```text
//! Input: [[1,3],[2,6],[8,10],[15,18]]
//! Output: [[1,6],[8,10],[15,18]]
//! Explanation: Since intervals [1,3] and [2,6] overlaps, merge them into [1,6].
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: [[1,4],[4,5]]
//! Output: [[1,5]]
//! Explanation: Intervals [1,4] and [4,5] are considered overlapping.
//! ```
//!
//! **NOTE**: 
//!
//! input types have been changed on April 15, 2019.
//!
//! Please reset to default code definition to get new method signature.
//!



pub type Input  = Vec<Vec<i32>>;
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn merge(&self, intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Sorted and Combine Interval.
pub struct Solution0;
impl Solution for Solution0 {

    fn merge(&self, mut intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        
        // sort by first element
        intervals.sort_by(|v1, v2| v2[0].cmp(&v1[0]));

        let mut ans = Vec::new();
        
        loop {
            // treat `intervals` as a stack
            let interval1 = if let Some(interval) = intervals.pop() {
                interval
            } else {
                return ans
            };

            let interval2 = if let Some(interval) = intervals.pop() {
                interval
            } else { 
                ans.push(interval1);
                return ans
            };


            if interval1[0] == interval2[0] {
                intervals.push(vec![interval1[0], interval1[1].max(interval2[1])]);
            } else {
                match interval1[1].cmp(&interval2[0]) {
                    | std::cmp::Ordering::Less => {
                        ans.push(interval1);
                        intervals.push(interval2);
                    },
                    | std::cmp::Ordering::Equal => {
                        intervals.push(vec![interval1[0], interval2[1]]);
                    },
                    | std::cmp::Ordering::Greater => {
                        if interval1[1] > interval2[1] {
                            intervals.push(interval1);
                        } else {
                            intervals.push(vec![interval1[0], interval2[1]]);
                        }
                    },
                }
            }

            dbg!(&intervals);
        }
        
    }
}
// -----------------------------------------------------------------------------

