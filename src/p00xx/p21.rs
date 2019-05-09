//!
//! Merge Two Sorted Lists
//!
//! https://leetcode.com/problems/merge-two-sorted-lists/
//!
//! Merge two sorted linked lists and return it as a new list.
//!
//! The new list should be made by splicing together the nodes of the first two lists.
//!
//! **Example:**
//! ```text
//! Input: 1->2->4, 1->3->4
//! Output: 1->1->2->3->4->4
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


#[derive(Debug, Clone)]
pub struct Input {
    pub l1: Option<Box<ListNode>>,
    pub l2: Option<Box<ListNode>>,
}
pub type Output = Option<Box<ListNode>>;

pub trait Solution {
    fn merge_two_lists(&self, l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>; 
}

// -----------------------------------------------------------------------------
/// Approach 0: Merge sort.
pub struct Solution0;
impl Solution for Solution0 {

    fn merge_two_lists(&self, mut l1: Option<Box<ListNode>>, mut l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

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

