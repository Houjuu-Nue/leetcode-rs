//! 
//! https://leetcode.com/problems/add-two-numbers/
//! 
//! You are given two non-empty linked lists representing two non-negative integers.
//! The digits are stored in reverse order and each of their nodes contain a single digit.
//! Add the two numbers and return it as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! # Example:
//! ```ignore
//! Input: (2 -> 4 -> 3) + (5 -> 6 -> 4)
//! Output: 7 -> 0 -> 8
//! Explanation: 342 + 465 = 807.
//! ```
//!

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {

    #[inline]
    pub fn new(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let node = ListNode { next, val };
        Some(Box::new(node))
    }
}

#[derive(Debug, Clone)]
pub struct Input {
    pub l1: Option<Box<ListNode>>,
    pub l2: Option<Box<ListNode>>,
}

pub type Answer = Option<Box<ListNode>>;

pub trait Solution {
    fn add_two_numbers(&self, l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> Answer;
}

pub struct Solution1;

impl Solution for Solution1 {

    fn add_two_numbers(&self, l1: &Option<Box<ListNode>>, l2: &Option<Box<ListNode>>) -> Answer {

        
        ListNode::new(7, ListNode::new(0, ListNode::new(8, None)))
    }
}
