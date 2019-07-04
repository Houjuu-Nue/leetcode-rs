//!
//! Insert Interval
//!
//! https://leetcode.com/problems/insert-interval/
//!
//! Given a set of *non-overlapping intervals*, insert a new interval into the intervals (merge if necessary).
//!
//! You may assume that the intervals were initially sorted according to their start times.
//!
//! **Example 1:**
//! ```text
//! Input: intervals = [[1,3],[6,9]], newInterval = [2,5]
//! Output: [[1,5],[6,9]]
//! ```
//!
//! **Example 2:**
//! ```text
//! Input: intervals = [[1,2],[3,5],[6,7],[8,10],[12,16]], newInterval = [4,8]
//! Output: [[1,2],[3,10],[12,16]]
//! Explanation: Because the new interval [4,8] overlaps with [3,5],[6,7],[8,10].
//! ```
//!
//! NOTE:
//!
//! input types have been changed on April 15, 2019.
//!
//! Please reset to default code definition to get new method signature.
//!


#[derive(Debug, Clone)]
pub struct Input {
    pub intervals: Vec<Vec<i32>>,
    pub new_interval: Vec<i32>,
}
pub type Output = Vec<Vec<i32>>;

pub trait Solution {
    fn insert(&self, intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Insert then Merge.
pub struct Solution0;
impl Solution for Solution0 {

    fn insert(&self, mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {

        let insert_loc = intervals.iter()
            .position(|interval| interval[0] > new_interval[0])
            .unwrap_or(intervals.len());
        intervals.insert(insert_loc, new_interval);

        merge(intervals)
    }
}

fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {

    let mut ans = Vec::new();
    let mut max = intervals[0].clone();

    for interval in intervals.into_iter().skip(1) {
        if interval[0] > max[1] {
            ans.push(max.clone());
            max = interval;
        } else if interval[1] > max[1] {
            max[1] = interval[1];
        }
    }

    ans.push(max);
    ans
}
// -----------------------------------------------------------------------------
