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
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut last = None;
        let mut cur = head;
        while let Some(inner_cur) = cur.as_deref_mut() {
            let temp = inner_cur.next.take();
            inner_cur.next = last;
            last = cur;
            cur = temp;
        }
        last
    }
}

#[cfg(test)]
mod tests_206 {
    use super::*;

    #[test]
    fn it_works() {
        let l = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
        let rev_l = Solution::reverse_list(l).unwrap();
        assert_eq!(rev_l.to_vec(), vec![5, 4, 3, 2, 1]);

        let l = ListNode::from_vec(vec![]);
        assert_eq!(Solution::reverse_list(l), None);
    }
}
