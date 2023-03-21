#![allow(dead_code)]

use std::collections::BinaryHeap;
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
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        for l in lists {
            head = Solution::merge_two_lists(head, l);
        }
        head
    }

    pub fn merge_k_lists_divide_and_conqure(
        lists: Vec<Option<Box<ListNode>>>,
    ) -> Option<Box<ListNode>> {
        let mut lists = lists;
        while lists.len() != 1 {
            lists = lists
                .chunks_mut(2)
                .map(|v| {
                    if v.len() == 2 {
                        Solution::merge_two_lists(v[0].take(), v[1].take())
                    } else {
                        v[0].take()
                    }
                })
                .collect()
        }
        lists.pop().unwrap_or(None)
    }

    pub fn merge_k_lists_max_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for list in lists {
            let mut node = list;
            while let Some(inner_node) = node {
                heap.push(inner_node.val);
                node = inner_node.next;
            }
        }
        let mut head = None;
        while let Some(v) = heap.pop() {
            let mut temp = ListNode::new(v);
            temp.next = head;
            head = Some(Box::new(temp));
        }
        head
    }

    pub fn merge_k_lists_min_heap(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut heap = BinaryHeap::new();
        for list in lists.into_iter().flatten() {
            heap.push(list);
        }
        let mut head = ListNode::new(0);
        let mut cur = &mut head;
        while let Some(mut l) = heap.pop() {
            if let Some(n) = l.next.take() {
                heap.push(n);
            }
            cur.next = Some(l);
            cur = cur.next.as_mut().unwrap();
        }
        head.next
    }

    fn merge_two_lists(
        mut l1: Option<Box<ListNode>>,
        mut l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (l1.as_ref(), l2.as_ref()) {
            (Some(n1), Some(n2)) => {
                if n1.val <= n2.val {
                    let next = l1.as_mut().unwrap().next.take();
                    l1.as_mut().unwrap().next = Solution::merge_two_lists(next, l2);
                    l1
                } else {
                    let next = l2.as_mut().unwrap().next.take();
                    l2.as_mut().unwrap().next = Solution::merge_two_lists(next, l1);
                    l2
                }
            }
            _ => l1.or(l2),
        }
    }
}

impl Ord for ListNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.val.cmp(&self.val)
    }
}

impl PartialOrd for ListNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests_23 {
    use super::*;

    #[test]
    fn it_works() {
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
            .into_iter()
            .map(|v| ListNode::from_vec(v))
            .collect();
        assert_eq!(
            Solution::merge_k_lists(lists).unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
            .into_iter()
            .map(|v| ListNode::from_vec(v))
            .collect();
        assert_eq!(
            Solution::merge_k_lists_divide_and_conqure(lists)
                .unwrap()
                .to_vec(),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
            .into_iter()
            .map(|v| ListNode::from_vec(v))
            .collect();
        assert_eq!(
            Solution::merge_k_lists_max_heap(lists).unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
        let lists = vec![vec![1, 4, 5], vec![1, 3, 4], vec![2, 6]]
            .into_iter()
            .map(|v| ListNode::from_vec(v))
            .collect();
        assert_eq!(
            Solution::merge_k_lists_min_heap(lists).unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4, 5, 6]
        );
    }
}
