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
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut head = Some(Box::new(ListNode::new(0)));
        let mut cur = &mut head;
        while list1.is_some() && list2.is_some() {
            if let Some(mut n1) = list1.take() {
                if let Some(mut n2) = list2.take() {
                    if let Some(inner_cur) = cur {
                        if n1.val < n2.val {
                            list1 = n1.next.take();
                            list2 = Some(n2);
                            inner_cur.next = Some(n1);
                        } else {
                            list2 = n2.next.take();
                            list1 = Some(n1);
                            inner_cur.next = Some(n2);
                        }
                        cur = &mut inner_cur.next;
                    }
                }
            }
        }
        if let Some(inner_cur) = cur {
            if list1.is_some() {
                inner_cur.next = list1
            }
            if list2.is_some() {
                inner_cur.next = list2
            }
        }
        if let Some(head) = head {
            head.next
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests_21 {
    use super::*;

    #[test]
    fn it_works() {
        let l1 = ListNode::from_vec(vec![1, 2, 4]);
        let l2 = ListNode::from_vec(vec![1, 3, 4]);
        assert_eq!(
            Solution::merge_two_lists(l1, l2).unwrap().to_vec(),
            vec![1, 1, 2, 3, 4, 4]
        );
    }
}
