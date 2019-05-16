//!
//! Swap Nodes in Pairs
//!
//! https://leetcode.com/problems/swap-nodes-in-pairs/
//!
//! Given a linked list, swap every two adjacent nodes and return its head.
//!
//! You may **not** modify the values in the list's nodes, only nodes itself may be changed.
//!
//! **Example:**
//! ```text
//! Given 1->2->3->4, you should return the list as 2->1->4->3.
//! ```
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


pub type Input  = Option<Box<ListNode>>;
pub type Output = Option<Box<ListNode>>;

pub trait Solution {
    fn swap_pairs(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Swap node during traversal.
pub struct Solution0;
impl Solution for Solution0 {

    fn swap_pairs(&self, head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

        let mut new_head = ListNode::new(0);
        new_head.next = head;
        let mut head = Some(Box::new(new_head));

        let mut p: &mut Option<Box<ListNode>> = &mut head;

        while let Some(ref mut pre) = p {

            // dbg!(pre.as_ref());

            if let Some(mut first) = pre.next.take() {
                if let Some(mut second) = first.next.take() {

                    let after = second.next.take();
                    first.next  = after;
                    second.next = Some(first);
                    pre.next    = Some(second);

                    p = &mut p.as_mut().unwrap().next.as_mut().unwrap().next;
                } else {
                    pre.next = Some(first);
                    p = &mut p.as_mut().unwrap().next;
                }
            } else {
                break
            }
        }

        head.unwrap().next.take()
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

