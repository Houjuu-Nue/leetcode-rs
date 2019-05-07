//!
//! Remove Nth Node From End of List!
//!
//! https://leetcode.com/problems/remove-nth-node-from-end-of-list/
//!
//! Given a linked list, remove the n-th node from the end of list and return its head.
//!
//! **Example:**.
//! ```text.
//! Given linked list: 1->2->3->4->5, and n = 2.
//!
//! After removing the second node from the end, the linked list becomes 1->2->3->5.
//! ```
//!
//! **Note:**
//!
//! Given n will always be valid.
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
    pub n: i32,
}
pub type Output = Option<Box<ListNode>>;

pub trait Solution {

    fn remove_nth_from_end(&self, head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>>;
}

// -----------------------------------------------------------------------------
/// Approach 0: Two pass algorithm 
pub struct Solution0;
impl Solution for Solution0 {

    fn remove_nth_from_end(&self, mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        
        let list_length = {
            let mut length = 0;
            let mut pointer = &head;
            while let Some(ref node) = pointer {
                length += 1;
                pointer = &node.next;
            }

            length
        };

        let delete_index = list_length - n as usize;

        let mut node_ref = &mut head;

        if delete_index > 0 {
            for _ in 0..(delete_index - 1) {
                node_ref = &mut node_ref.as_mut().unwrap().next;
            }
             
            let mut node_delete = node_ref.as_mut().unwrap().next.take();
            node_ref.as_mut().unwrap().next = if let Some(ref mut node) = node_delete {
                node.next.take()
            } else {
                None
            };

            head
        } else {
            head.unwrap().next
        }
    }
}
// -----------------------------------------------------------------------------


// -----------------------------------------------------------------------------
/// Approach 1: One pass algorithm. 
pub struct Solution1;
impl Solution for Solution1 {

    fn remove_nth_from_end(&self, head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {

        let mut p1 = &head;
        let mut p2 = &head;

        for _ in 0..=n {
            if let Some(ref p) = p1 {
                p1 = &p.next;
            } else {
                return head.unwrap().next
            }
        }

        while let Some(ref node) = p1 {
            p1 = &node.next;
            p2 = &p2.as_ref().unwrap().next;
        }

        let p_mut = p2 as *const Option<Box<ListNode>> as *mut Option<Box<ListNode>>;
        let p_mut = unsafe { &mut *p_mut };

        let node_delete = p_mut.as_mut().unwrap().next.take();
        p_mut.as_mut().unwrap().next = node_delete.unwrap().next.take();

        head
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

