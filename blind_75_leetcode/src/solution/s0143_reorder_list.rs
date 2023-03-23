#![allow(dead_code)]

use std::collections::VecDeque;
pub struct Solution {}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    fn from_vec(v: Vec<i32>) -> Option<Box<ListNode>> {
        let mut head: Option<Box<ListNode>> = None;
        let mut cur = &mut head;
        for i in v {
            let next = Box::new(ListNode::new(i));
            if let Some(inner_cur) = cur {
                inner_cur.next = Some(next);
                cur = &mut inner_cur.next;
            } else {
                *cur = Some(next)
            }
        }
        head
    }

    fn to_vec(&self) -> Vec<i32> {
        let mut cur = self;
        let mut v = vec![];
        while let Some(next) = &cur.next {
            v.push(cur.val);
            cur = next;
        }
        v.push(cur.val);
        v
    }
}

impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut queue = VecDeque::new();
        let mut cur = head.take();
        while let Some(node) = cur.as_mut() {
            let temp = node.next.take();
            queue.push_back(cur);
            cur = temp;
        }
        let mut dummy = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut dummy;
        while !queue.is_empty() {
            match (queue.pop_front(), queue.pop_back()) {
                (Some(mut fisrt), Some(last)) => {
                    fisrt.as_mut().unwrap().next = last;
                    cur.as_mut().unwrap().next = fisrt;
                    cur = &mut cur.as_mut().unwrap().next.as_mut().unwrap().next;
                }
                (Some(last), _) => {
                    cur.as_mut().unwrap().next = last;
                }
                _ => (),
            }
        }
        *head = dummy.unwrap().next.take();
    }
}

pub struct Solution2 {}

impl Solution2 {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let middle = Solution2::reverse_list(Solution2::middle_list(head));
        Solution2::merger_list(head, middle);
    }

    fn list_length(head: &Option<Box<ListNode>>) -> usize {
        let mut count = 0;
        let mut cur = head;
        while let Some(node) = cur {
            cur = &node.next;
            count += 1;
        }
        count
    }

    fn middle_list(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let count = Solution2::list_length(head);

        let mut cur = head;
        for _ in 0..count / 2 + 1 {
            // maker sure head is longer than other
            cur = &mut cur.as_mut().unwrap().next;
        }
        cur.take()
    }

    fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy = None;
        while let Some(mut node) = head.take() {
            let temp = node.next.take();
            node.next = dummy;
            head = temp;
            dummy = Some(node);
        }
        dummy
    }

    fn merger_list(head: &mut Option<Box<ListNode>>, mut other: Option<Box<ListNode>>) {
        let mut head_ptr = head;
        while head_ptr.is_some() && other.is_some() {
            let temp = other.as_mut().unwrap().next.take();
            other.as_mut().unwrap().next = head_ptr.as_mut().unwrap().next.take();
            head_ptr.as_mut().unwrap().next = other;
            head_ptr = &mut head_ptr.as_mut().unwrap().next.as_mut().unwrap().next;
            other = temp;
        }
    }
}

#[cfg(test)]
mod tests_143 {
    use super::*;

    #[test]
    fn it_works() {
        let mut head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        Solution::reorder_list(&mut head);
        assert_eq!(head.unwrap().to_vec(), vec![1, 5, 2, 4, 3]);

        let mut head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        Solution2::reorder_list(&mut head);
        assert_eq!(head.unwrap().to_vec(), vec![1, 5, 2, 4, 3]);
    }
}
