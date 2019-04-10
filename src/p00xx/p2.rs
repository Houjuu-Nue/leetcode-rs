//! 
//! Add Two Numbers
//! 
//! https://leetcode.com/problems/add-two-numbers/
//!
//! You are given two non-empty linked lists representing two non-negative integers.
//! The digits are stored in reverse order and each of their nodes contain a single digit.
//! Add the two numbers and return it as a linked list.
//!
//! You may assume the two numbers do not contain any leading zero, except the number 0 itself.
//!
//! ## Example:
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
    pub fn new2(val: i32, next: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let node = ListNode { next, val };
        Some(Box::new(node))
    }

    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}


#[derive(Debug, Clone)]
pub struct Input {
    pub l1: Option<Box<ListNode>>,
    pub l2: Option<Box<ListNode>>,
}

pub type Answer = Option<Box<ListNode>>;

pub trait Solution {
    fn add_two_numbers(&self, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

/// Approach 0: Elementary Math.
pub struct Solution0;
impl Solution for Solution0 {

    fn add_two_numbers(&self, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut digits = vec![];
        let mut carry = 0;

        let mut n1 = &l1;
        let mut n2 = &l2;

        // calculate sum of two number, store result into `digits` variable.
        loop {

            let (n1_val, n2_val) = match (n1, n2) {
                | (Some(node1), Some(node2)) => {
                    let n1_val = node1.val;
                    let n2_val = node2.val;
                    n1 = &node1.next;
                    n2 = &node2.next;

                    (n1_val, n2_val)
                },
                | (None, None) => {
                    if carry == 1 {
                        digits.push(carry);
                    }
                    break
                },
                | (Some(node1), None) => {
                    let n1_val = node1.val;
                    n1 = &node1.next;

                    (n1_val, 0)
                },
                | (None, Some(node2)) => {
                    let n2_val = node2.val;
                    n2 = &node2.next;

                    (0, n2_val)
                },
            };

            let digit_sum = n1_val + n2_val + carry;
            carry = digit_sum / 10;
            digits.push(digit_sum % 10);
        }
        
        // dbg!(digits);

        let mut result = None;
        let mut link_ref = &mut result;

        for remainder in digits {
            
            (*link_ref) = Some(Box::new(ListNode::new(remainder)));
            link_ref = &mut link_ref.as_mut().unwrap().next;
        }

        result
    }
}


impl ListNode {

    pub fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {

        let mut root = None;
        let mut node_ref = &mut root;
        for digit in v {
            (*node_ref) = ListNode::new2(digit, None);
            node_ref = &mut node_ref.as_mut().unwrap().next;
        }
        root
    }
}

pub fn list_node_to_vec(list: &Option<Box<ListNode>>) -> Vec<i32> {

    let mut result = vec![];

    let mut node_ref = list;
    while let Some(ref node) = node_ref {
        result.push(node.val);
        node_ref = &node.next;
    }

    result
}
