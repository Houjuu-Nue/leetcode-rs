//!
//! Merge k Sorted Lists
//!
//! https://leetcode.com/problems/merge-k-sorted-lists/
//!
//! Merge k sorted linked lists and return it as one sorted list.
//!
//! Analyze and describe its complexity.
//!
//! **Example:**
//! ```text
//! Input:
//! [
//!   1->4->5,
//!   1->3->4,
//!   2->6
//! ]
//! Output: 1->1->2->3->4->4->5->6
//! ```
//!
//!


// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {

    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}


pub type Input  = Vec<Option<Box<ListNode>>>;
pub type Output = Option<Box<ListNode>>;

pub trait Solution {
    fn merge_k_lists(&self, lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Merge list one by one. 
pub struct Solution0;
impl Solution for Solution0 {

    fn merge_k_lists(&self, lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {

        lists.into_iter().fold(None, |acc, list| merge_two_lists(acc, list))
    }
}

fn merge_two_lists(mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

    let mut new_list = None;
    let mut new_ref = &mut new_list;

    while l1.is_some() && l2.is_some() {
        let l1_value = l1.as_ref().unwrap().val.clone();
        let l2_value = l2.as_ref().unwrap().val.clone();

        if l1_value < l2_value {
            (*new_ref) = Some(Box::new(ListNode::new(l1_value)));
            l1 = l1.unwrap().next;
        } else {
            (*new_ref) = Some(Box::new(ListNode::new(l2_value)));
            l2 = l2.unwrap().next;
        }

        new_ref = &mut new_ref.as_mut().unwrap().next;
    }

    while let Some(v) = l1 {
        (*new_ref) = Some(Box::new(ListNode::new(v.val)));
        l1 = v.next;
        new_ref = &mut new_ref.as_mut().unwrap().next;
    }
    
    while let Some(v) = l2 {
        (*new_ref) = Some(Box::new(ListNode::new(v.val)));
        l2 = v.next;
        new_ref = &mut new_ref.as_mut().unwrap().next;
    }

    new_list
}
// -----------------------------------------------------------------------------


impl ListNode {

    pub fn from_list(v: &[i32]) -> Option<Box<ListNode>> {

        let mut root = None;
        let mut node_ref = &mut root;
        for &digit in v {
            (*node_ref) = Some(Box::new(ListNode::new(digit)));
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

