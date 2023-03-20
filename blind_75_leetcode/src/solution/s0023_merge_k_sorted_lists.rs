#![allow(dead_code)]
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
    // This is a template
    pub fn merge_k_lists(lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let mut head = None;
        for l in lists {
            head = Solution::merge_two_lists(head, l);
        }
        head
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
    }
}
