//!
//! Reverse Nodes in k-Group
//!
//! https://leetcode.com/problems/reverse-nodes-in-k-group/
//!
//! Given a linked list, reverse the nodes of a linked list k at a time and return its modified list.
//!
//! k is a positive integer and is less than or equal to the length of the linked list.
//!
//! If the number of nodes is not a multiple of k then left-out nodes in the end should remain as it is.
//!
//! **Example:**
//!
//! Given this linked list: `1->2->3->4->5`
//!
//! For k = 2, you should return: `2->1->4->3->5`.
//!
//! For k = 3, you should return: `3->2->1->4->5`.
//!
//! **Note:**
//!
//! - Only constant extra memory is allowed.
//!	- You may not alter the values in the list's nodes, only nodes itself may be changed.
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
    pub head: Option<Box<ListNode>>,
    pub k: i32,
}
pub type Output = Option<Box<ListNode>>;

pub trait Solution {
    fn reverse_k_group(&self, head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Traversal and Recursion.
pub struct Solution0;
impl Solution for Solution0 {

    fn reverse_k_group(&self, head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        reverse(head, k as usize)
    }
}

fn split_k(mut node: Option<Box<ListNode>>, k: usize) -> (Option<Box<ListNode>>, Option<Box<ListNode>>, bool) {
    
    let mut p = &mut node;
    for _ in 0..k {
        if let Some(n) = p {
            p = &mut n.next;
        } else {
            return (node, None, false)
        }
    }

    let remain_list = p.take();
    (node, remain_list, true)
}

fn reverse(node: Option<Box<ListNode>>, k: usize) -> Option<Box<ListNode>> {

    match split_k(node, k) {
        | (head, _, false) => {
            head
        },
        | (mut head, remain, _) => {

            // reverse the element starting from (k + 1)th element.
            let mut tmp_list: Option<Box<ListNode>> = reverse(remain, k);

            // reverse k element
            while let Some(mut node) = head.take() {

                head = node.next.take();
                
                node.next = tmp_list;
                tmp_list = Some(node);
            }

            tmp_list
        },
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

